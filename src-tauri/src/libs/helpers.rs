use std::path;

use config;

use crate::globals;

pub async fn get_freshclam_config() -> Result<config::freshclam::Config, String> {
    let config_directory_path_mutex_guard = globals::CONFIG_DIRECTORY_PATH.lock().await;
    let config_directory_path = config_directory_path_mutex_guard.clone();
    let freshclam_config_file_path_buf = config_directory_path.join("freshclam.conf");
    let freshclam_config_file_path = freshclam_config_file_path_buf.as_path();

    match config::freshclam::Config::from_file(freshclam_config_file_path) {
        Ok(config) => Ok(config),
        Err(err) => return Err(format!("Failed to read freshclam configuration file: {}", err)),
    }
}

pub async fn get_freshclam_config_path_buf() -> path::PathBuf {
    let config_directory_path_mutex_guard = globals::CONFIG_DIRECTORY_PATH.lock().await;
    let config_directory_path = config_directory_path_mutex_guard.clone();

    config_directory_path.join("freshclam.conf")
}

pub async fn get_freshclam_config_path_as_string() -> Result<String, String> {
    let freshclam_config_file_path_buf = get_freshclam_config_path_buf().await;

    match freshclam_config_file_path_buf.as_path().to_str() {
        Some(path) => Ok(path.to_string()),
        None => return Err("Failed to convert freshclam configuration file path to string".to_string()),
    }
}
