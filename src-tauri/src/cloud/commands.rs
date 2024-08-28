use std::process::Stdio;
use tauri::{AppHandle, Manager, State};
use tokio::process::Command as TokioCommand;

use crate::debug;

use super::*;

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_cloud_state(
    app_handle: AppHandle,
    shared_state: State<'_, state::CloudSharedState>,
) -> Result<(), ()> {
    debug!("get_cloud_state()", "Command call.");

    let mut public_state_mutex_guard = shared_state.0.public.lock().await;

    let (status, logs) = utils::get_service_status().await;
    let next_public_state = state::CloudPublicState {
        is_ready: true,
        is_running: public_state_mutex_guard.is_running,
        logs,
        status,
    };
    *public_state_mutex_guard = next_public_state.clone();
    app_handle.emit_all("cloud:state", &next_public_state).unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_cloud_update(app_handle: AppHandle) -> Result<(), ()> {
    use tauri::api::process::{Command, CommandEvent};

    use crate::globals;

    debug!("start_cloud_update()", "Command call.");

    let config_directory_path_mutex_guard = globals::CONFIG_DIRECTORY_PATH.lock().await;

    let config_directory_path = config_directory_path_mutex_guard.clone();
    let freshclam_config_file_path = config_directory_path.join("freshclam.conf");
    let freshclam_config_file_path_as_str = freshclam_config_file_path.to_str().unwrap();

    let (mut rx, _child) = Command::new_sidecar("freshclam")
        .expect("failed to create `freshclam` binary command")
        .args(["--config-file", freshclam_config_file_path_as_str])
        .spawn()
        .expect("Failed to spawn sidecar");

    let app_handle_traveller = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            let mut public_state_mutex_guard = app_handle_traveller
                .state::<state::CloudSharedState>()
                .inner()
                .0
                .public
                .lock()
                .await;

            if let CommandEvent::Stdout(ref line) = event {
                #[cfg(debug_assertions)]
                {
                    println!("[CommandEvent::Stdout] {}", line);
                }

                public_state_mutex_guard.logs.push(line.to_string());
            }

            if let CommandEvent::Stderr(ref line) = event {
                #[cfg(debug_assertions)]
                {
                    println!("[CommandEvent::Stderr] {}", line);
                }

                public_state_mutex_guard.logs.push(line.to_string());
            }

            app_handle_traveller
                .emit_all("cloud:state", &public_state_mutex_guard.clone())
                .unwrap();
        }
    });

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn start_cloud_daemon() -> () {
    debug!("start_cloud_daemon()", "Command call.");

    TokioCommand::new("systemctl")
        .args(["start", "clamav-freshclam"])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl stop freshclam`");
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn stop_cloud_daemon() -> () {
    debug!("stop_cloud_daemon()", "Command call.");

    TokioCommand::new("systemctl")
        .args(["stop", "clamav-freshclam"])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl stop freshclam`");
}
