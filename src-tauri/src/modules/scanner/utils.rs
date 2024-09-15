use super::*;

pub fn filter_log(log: String) -> bool {
    log.starts_with("Scanning ") || log.ends_with(": Empty file") || log.ends_with(": Access denied")
}

pub fn get_status_from_log(log: String, file_index: usize, total_files_length: usize) -> state::ScannerPublicState {
    let current_path: Option<String> = match true {
        true if log.starts_with("Scanning ") => Some(log.replace("Scanning ", "")),
        true if log.ends_with(": Empty file") => Some(log.replace(": Empty file", "")),
        true if log.ends_with(": Access denied") => Some(log.replace(": Access denied", "")),
        _ => None,
    };
    let progress = (file_index as f64 + f64::from(1)) / total_files_length as f64;

    if progress == 1 as f64 {
        return state::ScannerPublicState {
            current_path: None,
            progress: Some(progress),
            step: state::ScannerStatusStep::Idle,
        };
    }

    state::ScannerPublicState {
        current_path,
        progress: Some(progress),
        step: state::ScannerStatusStep::Running,
    }
}
