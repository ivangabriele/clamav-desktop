use std::{env, process::Stdio};
use tauri::{AppHandle, Manager, State};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

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
    app_handle
        .emit_all("cloud:state", &next_public_state)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_cloud_update(
    app_handle: AppHandle,
    shared_state: State<'_, state::CloudSharedState>,
) -> Result<(), ()> {
    debug!("start_cloud_update()", "Command call.");

    let is_dev_mode = env::var("TAURI_DEV").is_ok();

    // Update cloud state
    let mut public_state_mutex_guard = shared_state.0.public.lock().await;
    public_state_mutex_guard.is_running = true;
    app_handle
        .emit_all("cloud:state", &public_state_mutex_guard.clone())
        .unwrap();

    let mut command = if is_dev_mode {
        let mut cmd = Command::new("pkexec");
        cmd.args(["freshclam", "--daemon-notify"]);
        cmd
    } else {
        let mut cmd = Command::new("freshclam");
        cmd.args(["--daemon-notify"]);
        cmd
    };

    let mut child = command
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process.");

    let stdout = child
        .stdout
        .take()
        .expect("Failed to attach standard output.");

    let app_handle_clone_for_stdout = app_handle.clone();
    let app_handle_clone_for_end = app_handle.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            #[cfg(debug_assertions)]
            {
                println!("[libs::cli::run()] {}", line);
            }

            let mut public_state_mutex_guard = app_handle_clone_for_stdout
                .state::<state::CloudSharedState>()
                .inner()
                .0
                .public
                .lock()
                .await;
            public_state_mutex_guard.logs.push(line);

            app_handle_clone_for_stdout
                .emit_all("cloud:state", &public_state_mutex_guard.clone())
                .unwrap();
        }

        let _ = child.wait().await.expect("Failed to wait for child exit.");

        // Update the state to indicate the process is no longer running
        let mut public_state_mutex_guard = app_handle_clone_for_end
            .state::<state::CloudSharedState>()
            .inner()
            .0
            .public
            .lock()
            .await;
        public_state_mutex_guard.is_running = false;

        // Updated cloud state
        app_handle_clone_for_end
            .emit_all("cloud:state", &public_state_mutex_guard.clone())
            .unwrap();
    });

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn start_cloud_daemon() -> () {
    debug!("start_cloud_daemon()", "Command call.");

    Command::new("systemctl")
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

    Command::new("systemctl")
        .args(["stop", "clamav-freshclam"])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl stop freshclam`");
}
