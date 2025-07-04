use filer;
use std::{str, sync};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::debug;
use crate::error;
use crate::globals;

use super::*;

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_scanner_state(
    app_handle: AppHandle,
    shared_state: State<'_, state::ScannerSharedState>,
) -> Result<(), ()> {
    debug!("get_scanner_state()", "Command call.");

    let public_state_mutex_guard = shared_state.0.public.lock().await;
    app_handle
        .emit("scanner:state", &public_state_mutex_guard.clone())
        .unwrap();

    Ok(())
}

// TODO Add and use safe numeric type casting utils: https://stackoverflow.com/a/28280042/2736233.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_scanner(app_handle: AppHandle, paths: Vec<String>) -> Result<(), ()> {
    debug!("start_scanner()", "Command call.");

    state::set_public_state(
        &app_handle,
        state::ScannerPublicState {
            step: state::ScannerStatusStep::Listing,
            ..Default::default()
        },
    )
    .await;

    // Recursively count all the non-directory files within the selected paths
    let total_file_count = paths
        .to_owned()
        .into_iter()
        .map(|path| filer::file_list::count(true, Some(path), Some(filer::types::FileKind::File)))
        .sum::<usize>();
    debug!("start_scanner()", "Number of files to scan: {}.", total_file_count);

    state::set_public_state(
        &app_handle,
        state::ScannerPublicState {
            current_path: None,
            progress: None,
            step: state::ScannerStatusStep::Starting,
        },
    )
    .await;

    let local_data_directory_path_mutex_guard = globals::LOCAL_DATA_DIRECTORY_PATH.lock().await;
    let local_data_directory_path = local_data_directory_path_mutex_guard.clone();
    let local_data_directory_path_as_str = local_data_directory_path
        .to_str()
        .expect("Could not convert `local_data_directory_path` to string.");

    let args: Vec<String> = vec![
        // "clamscan".to_string(),
        "-rv".to_string(),
        format!("--database={}", local_data_directory_path_as_str).to_string(),
        "--follow-dir-symlinks=0".to_string(),
        "--follow-file-symlinks=0".to_string(),
        // "--gen-json=yes".to_string(),
        // "--leave-temps".to_string(),
    ]
    .into_iter()
    .chain(paths.to_owned().into_iter())
    .collect();
    let (mut rx, child) = app_handle
        .shell()
        .sidecar("clamscan")
        .expect("failed to create `my-sidecar` binary command")
        .args(args)
        .spawn()
        .expect("Failed to spawn sidecar");

    // Attach child ID to private state
    let mut child_id_mutex_guard = app_handle
        .state::<state::ScannerSharedState>()
        .inner()
        .0
        .private
        .child_id
        .lock()
        .await;
    *child_id_mutex_guard = Some(child.pid());
    // Attach child to private state
    let mut child_mutex_guard = app_handle
        .state::<state::ScannerSharedState>()
        .inner()
        .0
        .private
        .child
        .lock()
        .await;
    *child_mutex_guard = Some(child);

    let app_handle_traveller = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let mut file_index: usize = 0;
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(ref line) = event {
                let line_as_str = str::from_utf8(&line).expect("Failed to convert `line` to string.");

                debug!("start_scanner()", "{}", line_as_str);

                if utils::filter_log(line_as_str.to_owned()) {
                    let next_public_state =
                        utils::get_status_from_log(line_as_str.to_owned(), file_index, total_file_count);
                    state::set_public_state(&app_handle_traveller, next_public_state).await;

                    file_index += 1;
                }
            }

            if let CommandEvent::Stderr(ref line) = event {
                let line_as_str = str::from_utf8(&line).expect("Failed to convert `line` to string.");

                error!("start_scanner()", "{}", line_as_str);

                if utils::filter_log(line_as_str.to_owned()) {
                    let next_public_state =
                        utils::get_status_from_log(line_as_str.to_owned(), file_index, total_file_count);
                    state::set_public_state(&app_handle_traveller, next_public_state).await;

                    file_index += 1;
                }
            }
        }

        state::set_public_state(&app_handle_traveller, state::ScannerPublicState::default()).await;
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_scanner(app_handle: AppHandle, shared_state: State<'_, state::ScannerSharedState>) -> Result<(), ()> {
    debug!("stop_scanner()", "Command call.");

    // Enable `should_stop` flag
    shared_state
        .inner()
        .0
        .private
        .should_stop
        .store(true, sync::atomic::Ordering::SeqCst);

    let mut child_mutex_guard = shared_state.0.private.child.lock().await;
    if let Some(child) = child_mutex_guard.take() {
        debug!("stop_scanner()", "Killing child...");

        state::set_public_state(
            &app_handle,
            state::ScannerPublicState {
                current_path: None,
                progress: None,
                step: state::ScannerStatusStep::Stopping,
            },
        )
        .await;

        if let Err(e) = child.kill() {
            debug!("stop_scanner()", "Failed to kill scanner process: {:?}", e);

            return Err(());
        }

        *child_mutex_guard = None;
        shared_state
            .inner()
            .0
            .private
            .should_stop
            .store(false, sync::atomic::Ordering::SeqCst);

        state::set_public_state(&app_handle, state::ScannerPublicState::default()).await;

        debug!("stop_scanner()", "Child killed.");
    } else {
        debug!("stop_scanner()", "No child to kill.");
    }

    Ok(())
}
