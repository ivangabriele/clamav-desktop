use std::process::{Command as StdCommand, Stdio};
use std::str;

use regex::Regex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use tokio::process::Command as TokioCommand;

use crate::core;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum DaemonStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DaemonState {
    is_ready: bool,
    logs: Vec<String>,
    status: DaemonStatus,
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_daemon_state(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    let mut core_state_mutex_guard = state.0.lock().unwrap();

    let (status, logs) = get_service_status();
    let updated_daemon_state = DaemonState {
        is_ready: true,
        logs,
        status,
    };
    core_state_mutex_guard.daemon = updated_daemon_state;

    app_handle
        .emit_all("daemon:state", &core_state_mutex_guard.daemon)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_daemon() -> Result<(), ()> {
    println!("Calling command start_daemon().");

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
    println!("Calling command stop_daemon().");

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
