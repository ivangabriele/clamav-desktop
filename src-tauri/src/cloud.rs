use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use tauri::{AppHandle, Manager, State};

use crate::core;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum CloudDaemonStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CloudState {
    daemon_status: CloudDaemonStatus,
    is_ready: bool,
    is_running: bool,
    logs: Vec<String>,
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_cloud_state(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    let mut core_state_mutex_guard = state.0.lock().unwrap();

    let (daemon_status, logs) = get_service_status();
    core_state_mutex_guard.cloud.daemon_status = daemon_status;
    core_state_mutex_guard.cloud.is_ready = true;
    core_state_mutex_guard.cloud.logs = logs;

    app_handle
        .emit_all("cloud:state", &core_state_mutex_guard.cloud)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_cloud_update(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    use std::env;

    println!("Calling command start_update().");

    let is_dev_mode = env::var("TAURI_DEV").is_ok();

    // Update cloud state
    let mut core_state_mutex_guard = state.0.lock().unwrap();
    core_state_mutex_guard.cloud.is_running = true;
    app_handle
        .emit_all("cloud:state", &core_state_mutex_guard.cloud)
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
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        reader
            .lines()
            // TODO Is it the best way to achieve that?
            .filter_map(|line| line.ok())
            .for_each({
                move |line| {
                    #[cfg(debug_assertions)]
                    {
                        println!("[libs::cli::run()] {}", line);
                    }

                    let mut core_state_mutex_guard = app_handle_clone_for_stdout
                        .state::<core::state::SharedCoreState>()
                        .inner()
                        .0
                        .lock()
                        .unwrap();
                    core_state_mutex_guard.cloud.logs.push(line);

                    app_handle_clone_for_stdout
                        .emit_all("cloud:state", &core_state_mutex_guard.cloud)
                        .unwrap();
                }
            });

        let _ = child.wait().expect("Failed to wait for child exit.");

        // Update the state to indicate the process is no longer running
        let mut core_state_mutex_guard = app_handle_clone_for_end
            .state::<core::state::SharedCoreState>()
            .inner()
            .0
            .lock()
            .unwrap();
        core_state_mutex_guard.cloud.is_running = false;

        // Updated cloud state
        app_handle_clone_for_end
            .emit_all("cloud:state", &core_state_mutex_guard.cloud)
            .unwrap();
    });

    Ok(())
}

fn get_service_status() -> (CloudDaemonStatus, Vec<String>) {
    let output = Command::new("systemctl")
        .arg("--no-pager")
        .arg("status")
        .arg("clamav-freshclam")
        .output()
        .expect("Failed to retrieve systemctl status");

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

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn start_cloud_daemon() -> () {
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
    Command::new("systemctl")
        .args(["stop", "clamav-freshclam"])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl stop freshclam`");
}
