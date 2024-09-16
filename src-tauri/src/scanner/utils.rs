use std::sync::atomic::Ordering;
use tauri::{AppHandle, Manager};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::debug;

use super::*;

fn filter_log(log: String) -> bool {
    log.starts_with("Scanning ") || log.ends_with(": Empty file") || log.ends_with(": Access denied")
}

fn get_status_from_log(log: String, file_index: usize, total_files_length: usize) -> state::ScannerStatus {
    let progress = (file_index as f64 + f64::from(1)) / total_files_length as f64;
    let current_file_path: String;
    if log.starts_with("Scanning ") {
        current_file_path = log.replace("Scanning ", "");
    } else if log.ends_with(": Empty file") {
        current_file_path = log.replace(": Empty file", "");
    } else if log.ends_with(": Access denied") {
        current_file_path = log.replace(": Access denied", "");
    } else {
        current_file_path = "unknown".to_string();
    }

    if progress == 1 as f64 {
        return state::ScannerStatus {
            current_file_path: "Done".to_string(),
            progress,
            step: state::ScannerStatusStep::Idle,
        };
    }

    state::ScannerStatus {
        current_file_path,
        progress,
        step: state::ScannerStatusStep::Running,
    }
}

#[cfg(not(tarpaulin_include))]
pub async fn handle_scanner_output(app_handle: AppHandle, total_files_length: usize) -> () {
    let mut child_mutex_guard = app_handle
        .state::<state::SharedScannerState>()
        .inner()
        .0
        .private
        .child
        .lock()
        .await;

    if let Some(child_mut) = child_mutex_guard.as_mut() {
        let stdout = child_mut.stdout.take().unwrap();
        let mut file_index: usize = 0;
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if app_handle
                .state::<state::SharedScannerState>()
                .inner()
                .0
                .private
                .should_stop
                .load(Ordering::SeqCst)
            {
                debug!("handle_scanner_output()", "Stopped.");

                return;
            }

            debug!("handle_scanner_output()", "Output: `{}`.", line);

            if filter_log(line.to_owned()) {
                let next_status = get_status_from_log(line.to_owned(), file_index, total_files_length);
                app_handle.emit_all("scanner:status", next_status).unwrap();

                file_index += 1;
            }
        }
    }

    update_public_state(&app_handle, None, Some(false)).await;
}

#[cfg(not(tarpaulin_include))]
pub fn reset_status(app_handle: &AppHandle) -> () {
    app_handle
        .emit_all("scanner:status", state::ScannerStatus::default())
        .expect("Failed to emit `scanner:status` event.");
}

#[cfg(not(tarpaulin_include))]
pub async fn update_public_state(
    app_handle: &AppHandle,
    file_explorer_tree: Option<filer::file_explorer::FileExplorerTree>,
    is_running: Option<bool>,
) -> () {
    let mut public_state_mutex_guard = app_handle
        .state::<state::SharedScannerState>()
        .inner()
        .0
        .public
        .lock()
        .await;
    if let Some(file_explorer_tree) = file_explorer_tree {
        public_state_mutex_guard.file_explorer_tree = file_explorer_tree;
    }
    // TODO Manage that.
    public_state_mutex_guard.is_ready = true;
    if let Some(is_running) = is_running {
        public_state_mutex_guard.is_running = is_running;
    }

    app_handle
        .emit_all("scanner:state", public_state_mutex_guard.clone())
        .expect("Failed to emit `scanner:state` event.");
}

#[cfg(not(tarpaulin_include))]
pub async fn update_status(
    app_handle: &AppHandle,
    current_file_path: Option<String>,
    progress: Option<f64>,
    step: Option<state::ScannerStatusStep>,
) -> () {
    let next_status = state::ScannerStatus {
        current_file_path: current_file_path.unwrap_or(String::default()),
        progress: progress.unwrap_or(f64::default()),
        step: step.unwrap_or(state::ScannerStatusStep::default()),
    };

    app_handle
        .emit_all("scanner:status", next_status)
        .expect("Failed to emit `scanner:status` event.");
}
