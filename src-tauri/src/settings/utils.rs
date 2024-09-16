use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager};
use tokio::fs;
use walkdir::WalkDir;

use crate::error;

use super::*;

#[cfg(target_os = "linux")]
const DEFAULT_CLAMD_CONF_FILE_PATH: &str = "/etc/clamav/clamd.conf";
#[cfg(target_os = "macos")]
const DEFAULT_CLAMD_CONF_FILE_PATH: &str = "/usr/local/etc/clamav/clamd.conf";
#[cfg(target_os = "windows")]
const DEFAULT_CLAMD_CONF_FILE_PATH: &str = "C:\\ClamAV\\clamd.conf";

pub async fn get_clamd_conf_file_path() -> Option<String> {
    let default_file_path = PathBuf::from(DEFAULT_CLAMD_CONF_FILE_PATH);

    if fs::metadata(&default_file_path)
        .await
        .map(|m| m.is_file())
        .unwrap_or(false)
    {
        return default_file_path
            .to_str()
            .map(|file_path_as_str| file_path_as_str.to_string());
    }

    let search_result = tokio::task::spawn_blocking(|| {
        let mut found_file_paths = Vec::new();
        for entry in WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {
            if entry.file_name().to_string_lossy() == "clamd.conf" {
                found_file_paths.push(entry.into_path());
            }
        }
        found_file_paths
    })
    .await
    .unwrap_or_else(|_| Vec::new());

    match search_result.len() {
        0 => {
            error!("get_clamd_conf_file_path()", "No 'clamd.conf' files found.");

            None
        }
        1 => search_result
            .into_iter()
            .next()
            .unwrap()
            .to_str()
            .map(|file_path_as_str| file_path_as_str.to_string()),
        _ => {
            error!("get_clamd_conf_file_path()", "Multiple 'clamd.conf' files found.");

            None
        }
    }
}

#[cfg(not(tarpaulin_include))]
pub async fn get_debug_clamd_conf_file_path() -> Option<String> {
    let debug_clamd_conf_file_path = dev::get_debug_clamd_conf_file_path();

    if !Path::new(&debug_clamd_conf_file_path).exists() {
        let maybe_clamd_conf_file_path = get_clamd_conf_file_path().await;
        let debug_clamd_conf_file_source = match maybe_clamd_conf_file_path {
            Some(clamd_conf_file_path) => fs::read_to_string(&clamd_conf_file_path)
                .await
                .expect(format!("Could not read from `{}`.", debug_clamd_conf_file_path).as_str()),
            None => "".to_owned(),
        };

        fs::write(&debug_clamd_conf_file_path, debug_clamd_conf_file_source)
            .await
            .expect(format!("Could not write to `{}`.", debug_clamd_conf_file_path).as_str());
    }

    Some(debug_clamd_conf_file_path)
}

#[cfg(not(tarpaulin_include))]
pub async fn update_public_state(
    app_handle: &AppHandle,
    clamd_conf_file_path: Option<Option<String>>,
    clamd_conf_file_source: Option<Option<String>>,
    is_writing: Option<bool>,
) -> () {
    let mut public_state_mutex_guard = app_handle
        .state::<state::SharedSettingsState>()
        .inner()
        .0
        .public
        .lock()
        .await;
    if let Some(clamd_conf_file_path) = clamd_conf_file_path {
        public_state_mutex_guard.clamd_conf_file_path = clamd_conf_file_path;
    }
    if let Some(clamd_conf_file_source) = clamd_conf_file_source {
        public_state_mutex_guard.clamd_conf_file_source = clamd_conf_file_source;
    }
    // TODO Manage that.
    public_state_mutex_guard.is_ready = true;
    if let Some(is_writing) = is_writing {
        public_state_mutex_guard.is_writing = is_writing;
    }

    app_handle
        .emit("settings:state", public_state_mutex_guard.clone())
        .expect("Failed to emit `settings:state` event.");
}
