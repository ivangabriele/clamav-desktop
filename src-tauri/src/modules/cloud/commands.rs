use std::str;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::debug;
use crate::error;
use crate::globals;

use super::*;

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn check_cloud_update(app_handle: AppHandle) -> Result<(), ()> {
    debug!("check_cloud_update()", "Command call.");

    let local_version = match utils::get_local_version().await {
        Ok(maybe_version) => maybe_version,
        Err(err) => {
            error!("check_cloud_update()", "{}", err);

            return Err(());
        }
    };
    let remote_version = match utils::get_remote_version().await {
        Ok(version) => version,
        Err(err) => {
            error!("check_cloud_update()", "{}", err);

            return Err(());
        }
    };
    let is_up_to_date = remote_version.bytecode.is_some()
        && remote_version.daily.is_some()
        && remote_version.main.is_some()
        && local_version.bytecode.is_some()
        && local_version.daily.is_some()
        && local_version.main.is_some()
        && remote_version.bytecode == local_version.bytecode
        && remote_version.daily == local_version.daily
        && remote_version.main == local_version.main;

    println!("local_version: {:?}", local_version);
    println!("remote_version: {:?}", remote_version);

    state::patch_public_state(
        &app_handle,
        state::CloudPublicStatePatch {
            is_up_to_date: Some(Some(is_up_to_date)),
            ..Default::default()
        },
    )
    .await;

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_cloud_state(
    app_handle: AppHandle,
    shared_state: State<'_, state::CloudSharedState>,
) -> Result<(), ()> {
    debug!("get_cloud_state()", "Command call.");

    let public_state_mutex_guard = shared_state.0.public.lock().await;
    let public_state = &public_state_mutex_guard.clone();

    app_handle
        .emit("cloud:state", public_state)
        .map_err(|err| error!("get_cloud_state()", "{}", err))
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_cloud_update(app_handle: AppHandle) -> Result<(), ()> {
    use tauri::utils::platform::current_exe;

    use crate::libs;

    debug!("start_cloud_update()", "Command call.");

    state::patch_public_state(
        &app_handle,
        state::CloudPublicStatePatch {
            status: Some(globals::ModuleStatus::Running),
            ..Default::default()
        },
    )
    .await;

    let freshclam_config_file_path_as_string = match libs::helpers::get_freshclam_config_path_as_string().await {
        Ok(path) => path,
        Err(err) => return utils::handle_error(&app_handle, err).await,
    };

    let sidecar_path = current_exe().unwrap().parent().unwrap().join("freshclam");
    println!("sidecar_path: {:?}", sidecar_path);

    println!(
        "{} {}",
        sidecar_path.to_str().unwrap(),
        ["--config-file", freshclam_config_file_path_as_string.as_str()].join(" ")
    );

    let (mut rx, _child) = app_handle
        .shell()
        .sidecar("freshclam")
        .expect("failed to create `freshclam` binary command")
        .args(["--config-file", freshclam_config_file_path_as_string.as_str()])
        .spawn()
        .expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(ref line) = event {
                let line_as_str = str::from_utf8(&line).expect("Failed to convert `line` to string.");

                debug!("start_cloud_update()", "{}", line_as_str);
            }

            if let CommandEvent::Stderr(ref line) = event {
                let line_as_str = str::from_utf8(&line).expect("Failed to convert `line` to string.");

                error!("start_cloud_update()", "{}", line_as_str);
            }
        }

        state::set_public_state(
            &app_handle,
            state::CloudPublicState {
                status: globals::ModuleStatus::Stopped,
                is_up_to_date: Some(true),
            },
        )
        .await;
    });

    Ok(())
}
