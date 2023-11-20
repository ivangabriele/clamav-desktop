use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::{Command as StdCommand, Stdio};
use std::str;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tokio::process::Command as TokioCommand;
use tokio::sync::Mutex;

use crate::debug;

#[derive(Default)]
pub struct DaemonStateArcMutex(pub Arc<Mutex<DaemonState>>);

#[derive(Clone, Debug, Default, Serialize)]
pub struct DaemonState {
    is_ready: bool,
    logs: Vec<String>,
    status: DaemonStatus,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum DaemonStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_daemon_state(
    app_handle: AppHandle,
    state: State<'_, DaemonStateArcMutex>,
) -> Result<(), ()> {
    debug!("get_daemon_state()", "Command call.");

    let mut daemon_state_mutex_guard = state.0.lock().await;

    let (status, logs) = get_service_status();
    let next_daemon_state = DaemonState {
        is_ready: true,
        logs,
        status,
    };
    *daemon_state_mutex_guard = next_daemon_state.clone();
    app_handle
        .emit_all("daemon:state", &next_daemon_state)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_daemon() -> Result<(), ()> {
    debug!("start_daemon()", "Command call.");

    TokioCommand::new("systemctl")
        .args(["--no-pager", "start", "clamav-daemon"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl --no-pager stop clamav-daemon`");

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn stop_daemon() -> Result<(), ()> {
    debug!("stop_daemon()", "Command call.");

    TokioCommand::new("systemctl")
        .args(["--no-pager", "stop", "clamav-daemon"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl --no-pager stop clamav-daemon`");

    Ok(())
}

fn get_service_status() -> (DaemonStatus, Vec<String>) {
    let output = StdCommand::new("systemctl")
        .arg("--no-pager")
        .arg("status")
        .arg("clamav-daemon")
        .output()
        .expect("Failed to retrieve systemctl status");

    let output_str = str::from_utf8(&output.stdout).unwrap();

    let status_regex = Regex::new(r"Active: (\w+) \(").unwrap();

    let mut status = DaemonStatus::Unknown;
    let mut log = Vec::new();

    for line in output_str.lines() {
        if let Some(cap) = status_regex.captures(line) {
            status = match &cap[1] {
                "active" => DaemonStatus::Running,
                "inactive" => DaemonStatus::Stopped,
                _ => DaemonStatus::Unknown,
            };
        } else {
            log.push(line.to_string());
        }
    }

    (status, log)
}
