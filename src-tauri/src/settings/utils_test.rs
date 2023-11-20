use super::get_clamd_conf_file_path;
use std::{env, fs};
use tokio::fs::metadata;

async fn create_test_file_if_not_exists(path: &str) -> bool {
    if metadata(path).await.is_err() {
        let _ = fs::File::create(path);

        true
    } else {
        false
    }
}

async fn cleanup_test_file(path: &str) {
    let _ = fs::remove_file(path);
}

#[tokio::test]
async fn get_clamd_conf_file_path_returns_default_path_on_macos() {
    if cfg!(target_os = "macos") && env::var("CI").is_err() {
        let expected_path = "/usr/local/etc/clamav/clamd.conf";
        let is_dummy_file = create_test_file_if_not_exists(expected_path).await;

        let result = get_clamd_conf_file_path().await;
        assert_eq!(result, Some(expected_path.to_string()));

        if is_dummy_file {
            cleanup_test_file(expected_path).await;
        }
    }
}

#[tokio::test]
async fn get_clamd_conf_file_path_returns_default_path_on_unix() {
    if cfg!(target_os = "linux") {
        let expected_path = "/etc/clamav/clamd.conf";
        let is_dummy_file = create_test_file_if_not_exists(expected_path).await;

        let result = get_clamd_conf_file_path().await;
        assert_eq!(result, Some(expected_path.to_string()));

        if is_dummy_file {
            cleanup_test_file(expected_path).await;
        }
    }
}

#[tokio::test]
async fn get_clamd_conf_file_path_returns_default_path_on_windows() {
    if cfg!(target_os = "windows") && env::var("CI").is_err() {
        let expected_path = "C:\\ClamAV\\clamd.conf";
        let is_dummy_file = create_test_file_if_not_exists(expected_path).await;

        let result = get_clamd_conf_file_path().await;
        assert_eq!(result, Some(expected_path.to_string()));

        if is_dummy_file {
            cleanup_test_file(expected_path).await;
        }
    }
}
