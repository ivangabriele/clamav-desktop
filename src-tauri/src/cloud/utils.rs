use regex::Regex;
use tokio::process::Command;

use crate::debug;

use super::*;

pub async fn get_service_status() -> (state::CloudDaemonStatus, Vec<String>) {
    debug!("get_service_status()", "Call.");

    if cfg!(target_os = "linux") {
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
                    state::CloudDaemonStatus::Unknown,
                    vec!["Failed to retrieve systemctl status".to_string()],
                )
            }
        };
        let output_str = std::str::from_utf8(&output.stdout).unwrap();

        let status_regex = Regex::new(r"Active: (\w+) \(").unwrap();

        let mut status = state::CloudDaemonStatus::Unknown;
        let mut log = Vec::new();

        for line in output_str.lines() {
            if let Some(cap) = status_regex.captures(line) {
                status = match &cap[1] {
                    "active" => state::CloudDaemonStatus::Running,
                    "inactive" => state::CloudDaemonStatus::Stopped,
                    _ => state::CloudDaemonStatus::Unknown,
                };
            } else {
                log.push(line.to_string());
            }
        }

        return (status, log);
    }

    if cfg!(target_os = "macos") {
        let status = state::CloudDaemonStatus::Stopped;
        let log: Vec<String> = Vec::new();

        return (status, log);
    }

    if cfg!(target_os = "windows") {
        let status = state::CloudDaemonStatus::Stopped;
        let log: Vec<String> = Vec::new();

        return (status, log);
    }

    panic!("Unsupported OS.");
}
