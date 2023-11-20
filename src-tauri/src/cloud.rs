use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{env, process::Stdio, sync::Arc};
use tauri::{AppHandle, Manager, State};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;

use crate::debug;

#[derive(Default)]
pub struct CloudStateArcMutex(pub Arc<Mutex<CloudState>>);

#[derive(Clone, Debug, Default, Serialize)]
pub struct CloudState {
    status: CloudDaemonStatus,
    is_ready: bool,
    is_running: bool,
    logs: Vec<String>,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum CloudDaemonStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_cloud_state(
    app_handle: AppHandle,
    state: State<'_, CloudStateArcMutex>,
) -> Result<(), ()> {
    debug!("get_cloud_state()", "Command call.");

    let mut cloud_state_mutex_guard = state.0.lock().await;

    let (status, logs) = get_service_status().await;
    let next_cloud_state = CloudState {
        is_ready: true,
        is_running: cloud_state_mutex_guard.is_running,
        logs,
        status,
    };
    *cloud_state_mutex_guard = next_cloud_state.clone();
    app_handle
        .emit_all("cloud:state", &next_cloud_state)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_cloud_update(
    app_handle: AppHandle,
    state: State<'_, CloudStateArcMutex>,
) -> Result<(), ()> {
    debug!("start_cloud_update()", "Command call.");

    let is_dev_mode = env::var("TAURI_DEV").is_ok();

    // Update cloud state
    let mut cloud_state_mutex_guard = state.0.lock().await;
    cloud_state_mutex_guard.is_running = true;
    app_handle
        .emit_all("cloud:state", &cloud_state_mutex_guard.clone())
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

            let mut cloud_state_mutex_guard = app_handle_clone_for_stdout
                .state::<CloudStateArcMutex>()
                .inner()
                .0
                .lock()
                .await;
            cloud_state_mutex_guard.logs.push(line);

            app_handle_clone_for_stdout
                .emit_all("cloud:state", &cloud_state_mutex_guard.clone())
                .unwrap();
        }

        let _ = child.wait().await.expect("Failed to wait for child exit.");

        // Update the state to indicate the process is no longer running
        let mut cloud_state_mutex_guard = app_handle_clone_for_end
            .state::<CloudStateArcMutex>()
            .inner()
            .0
            .lock()
            .await;
        cloud_state_mutex_guard.is_running = false;

        // Updated cloud state
        app_handle_clone_for_end
            .emit_all("cloud:state", &cloud_state_mutex_guard.clone())
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

async fn get_service_status() -> (CloudDaemonStatus, Vec<String>) {
    debug!("get_service_status()", "Command call.");

    let output_result = Command::new("systemctl")
        .arg("--no-pager")
        .arg("status")
        .arg("clamav-freshclam")
        .output()
        .await;

    let output = match output_result {
        Ok(output) => output,
        // TODO Handle the error here.
        Err(_) => {
            return (
                CloudDaemonStatus::Unknown,
                vec!["Failed to retrieve systemctl status".to_string()],
            )
        }
    };
    let output_str = std::str::from_utf8(&output.stdout).unwrap();

    let status_regex = Regex::new(r"Active: (\w+) \(").unwrap();

    let mut status = CloudDaemonStatus::Unknown;
    let mut log = Vec::new();

    for line in output_str.lines() {
        if let Some(cap) = status_regex.captures(line) {
            status = match &cap[1] {
                "active" => CloudDaemonStatus::Running,
                "inactive" => CloudDaemonStatus::Stopped,
                _ => CloudDaemonStatus::Unknown,
            };
        } else {
            log.push(line.to_string());
        }
    }

    (status, log)
}
