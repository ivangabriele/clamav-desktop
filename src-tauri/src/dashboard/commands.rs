use std::process::Stdio;
use tauri::{AppHandle, Manager, State};
use tokio::process::Command;

use crate::debug;

use super::*;

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_dashboard_state(
    app_handle: AppHandle,
    shared_state: State<'_, state::DashboardSharedState>,
) -> Result<(), ()> {
    debug!("get_dashboard_state()", "Command call.");

    let mut public_state_mutex_guard = shared_state.0.public.lock().await;

    let (status, logs) = utils::get_service_status();
    let next_public_state = state::DashboardPublicState {
        is_ready: true,
        logs,
        status,
    };
    *public_state_mutex_guard = next_public_state.clone();
    app_handle
        .emit_all("dashboard:state", &next_public_state)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_daemon() -> Result<(), ()> {
    debug!("start_daemon()", "Command call.");

    Command::new("systemctl")
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

    Command::new("systemctl")
        .args(["--no-pager", "stop", "clamav-daemon"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl --no-pager stop clamav-daemon`");

    Ok(())
}
