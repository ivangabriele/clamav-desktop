use filer;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Manager, State};

use crate::debug;
use crate::libs;

use super::*;

// TODO Dry this function, most of it is already in `load_scanner_state()`.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_scanner_state(
    app_handle: AppHandle,
    shared_state: State<'_, state::SharedScannerState>,
) -> Result<(), ()> {
    debug!("get_scanner_state()", "Command call.");

    let public_state_mutex_guard = shared_state.0.public.lock().await;
    app_handle
        .emit_all("scanner:state", &public_state_mutex_guard.clone())
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn load_scanner_state(app_handle: AppHandle) -> Result<(), ()> {
    debug!("load_scanner_state()", "Command call.");

    let file_explorer_tree =
        filer::file_list::list::<String>(false, None, Some(filer::types::FileKind::Directory))
            .into_file_explorer()
            .into_tree();

    utils::update_public_state(&app_handle, Some(file_explorer_tree), None).await;

    Ok(())
}

// TODO Add and use safe numeric type casting utils: https://stackoverflow.com/a/28280042/2736233.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_scanner(
    app_handle: AppHandle,
    shared_state: State<'_, state::SharedScannerState>,
) -> Result<(), ()> {
    debug!("start_scanner()", "Command call.");

    let file_explorer_tree = {
        shared_state
            .inner()
            .0
            .public
            .lock()
            .await
            .file_explorer_tree
            .clone()
    };

    utils::update_public_state(&app_handle, None, Some(true)).await;
    utils::update_status(
        &app_handle,
        None,
        None,
        Some(state::ScannerStatusStep::Listing),
    )
    .await;

    // List selected paths
    let paths =
        filer::file_explorer::FileExplorer::new(file_explorer_tree.clone()).into_checked_paths();
    debug!(
        "start_scanner()",
        "Recursively listing files in {:?}...", paths
    );
    let args: Vec<String> = vec![
        // "clamscan".to_string(),
        "-rv".to_string(),
        "--follow-dir-symlinks=0".to_string(),
        "--follow-file-symlinks=0".to_string(),
        // "--gen-json=yes".to_string(),
        // "--leave-temps".to_string(),
    ]
    .into_iter()
    .chain(paths.to_owned().into_iter())
    .collect();

    utils::update_status(
        &app_handle,
        None,
        None,
        Some(state::ScannerStatusStep::Counting),
    )
    .await;

    // Recursively count all the non-directory files within the selected paths
    let total_files_length = paths
        .to_owned()
        .into_iter()
        .map(|path| filer::file_list::count(true, Some(path), Some(filer::types::FileKind::File)))
        .sum::<usize>();
    debug!(
        "start_scanner()",
        "Number of files to scan: {}.", total_files_length
    );

    utils::update_status(
        &app_handle,
        None,
        None,
        Some(state::ScannerStatusStep::Starting),
    )
    .await;

    let mut args_with_clamscan = vec!["clamscan".to_string()];
    args_with_clamscan.extend(args.iter().cloned());
    let child = libs::cli::run(String::from("clamscan"), args).await;
    child.id();

    // Attach child ID to private state
    let mut child_id_mutex_guard = app_handle
        .state::<state::SharedScannerState>()
        .inner()
        .0
        .private
        .child_id
        .lock()
        .await;
    *child_id_mutex_guard = child.id();
    // Attach child to private state
    let mut child_mutex_guard = app_handle
        .state::<state::SharedScannerState>()
        .inner()
        .0
        .private
        .child
        .lock()
        .await;
    *child_mutex_guard = Some(child);

    let app_handle_clone_for_log = app_handle.clone();
    tokio::spawn(utils::handle_scanner_output(
        app_handle_clone_for_log,
        total_files_length,
    ));

    Ok(())
}

#[tauri::command]
pub async fn stop_scanner(
    app_handle: AppHandle,
    shared_state: State<'_, state::SharedScannerState>,
) -> Result<(), ()> {
    debug!("stop_scanner()", "Command call.");

    // Enable `should_stop` flag
    shared_state
        .inner()
        .0
        .private
        .should_stop
        .store(true, Ordering::SeqCst);

    let mut child_mutex_guard = shared_state.0.private.child.lock().await;
    if let Some(child_mut) = child_mutex_guard.as_mut() {
        debug!("stop_scanner()", "Killing child...");

        utils::update_status(
            &app_handle,
            None,
            None,
            Some(state::ScannerStatusStep::Stopping),
        )
        .await;

        child_mut
            .kill()
            .await
            .expect("Failed to kill scanner process.");

        // Detach child from private state
        *child_mutex_guard = None;
        // Reset `should_stop` flag
        shared_state
            .inner()
            .0
            .private
            .should_stop
            .store(false, Ordering::SeqCst);

        utils::update_public_state(&app_handle, None, Some(false)).await;
        utils::reset_status(&app_handle);

        debug!("stop_scanner()", "Child killed.");
    } else {
        debug!("stop_scanner()", "No child to kill.");
    }

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn toggle_file_explorer_node_check(
    app_handle: AppHandle,
    index_path: Vec<usize>,
    shared_state: State<'_, state::SharedScannerState>,
) -> Result<(), ()> {
    debug!("toggle_file_explorer_node_check()", "Command call.");

    let mut public_state_mutex_guard = shared_state.0.public.lock().await;
    let mut next_file_explorer = filer::file_explorer::FileExplorer::new(
        public_state_mutex_guard.file_explorer_tree.to_owned(),
    );
    next_file_explorer.toggle_is_checked(index_path);
    public_state_mutex_guard.file_explorer_tree = next_file_explorer.into_tree().to_owned();
    app_handle
        .emit_all("scanner:state", &public_state_mutex_guard.clone())
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn toggle_file_explorer_node_expansion(
    app_handle: AppHandle,
    shared_state: State<'_, state::SharedScannerState>,
    index_path: Vec<usize>,
) -> Result<(), ()> {
    debug!("toggle_file_explorer_node_expansion()", "Command call.");

    let mut public_state_mutex_guard = shared_state.0.public.lock().await;
    let mut next_file_explorer = filer::file_explorer::FileExplorer::new(
        public_state_mutex_guard.file_explorer_tree.to_owned(),
    );
    next_file_explorer.toggle_is_expanded(index_path);
    public_state_mutex_guard.file_explorer_tree = next_file_explorer.into_tree().to_owned();
    app_handle
        .emit_all("scanner:state", &public_state_mutex_guard.clone())
        .unwrap();

    Ok(())
}
