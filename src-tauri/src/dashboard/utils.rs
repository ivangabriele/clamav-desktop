use super::*;

pub fn get_service_status() -> (state::DashboardStatus, Vec<String>) {
    if cfg!(target_os = "linux") {
        use regex::Regex;
        use std::process::Command;
        use std::str;
        let output = Command::new("systemctl")
            .arg("--no-pager")
            .arg("status")
            .arg("clamav-daemon")
            .output()
            .expect("Failed to retrieve systemctl status");

        let output_str = str::from_utf8(&output.stdout).unwrap();

        let status_regex = Regex::new(r"Active: (\w+) \(").unwrap();

        let mut status = state::DashboardStatus::Unknown;
        let mut log = Vec::new();

        for line in output_str.lines() {
            if let Some(cap) = status_regex.captures(line) {
                status = match &cap[1] {
                    "active" => state::DashboardStatus::Running,
                    "inactive" => state::DashboardStatus::Stopped,
                    _ => state::DashboardStatus::Unknown,
                };
            } else {
                log.push(line.to_string());
            }
        }

        return (status, log);
    }

    if cfg!(target_os = "macos") {
        let status = state::DashboardStatus::Stopped;
        let log: Vec<String> = Vec::new();

        return (status, log);
    }

    if cfg!(target_os = "windows") {
        let status = state::DashboardStatus::Stopped;
        let log: Vec<String> = Vec::new();

        return (status, log);
    }

    panic!("Unsupported OS.");
}
