use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use crate::{core, libs};
// use cli;
use filer;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ScannerStatusStep {
    /// Counting the files to scan.
    Counting,
    /// Default step (= waiting for a new job).
    Idle,
    /// Listing the files to scan.
    Listing,
    /// Scanning the files.
    Running,
    /// Starting (= has called `clamscan` CLI command).
    Starting,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScannerState {
    file_explorer_tree: filer::file_explorer::FileExplorerTree,
    is_ready: bool,
    pub is_running: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ScannerStatus {
    current_file_path: String,
    progress: f64,
    step: ScannerStatusStep,
}

pub const INITIAL_SCANNER_STATE: ScannerState = ScannerState {
    file_explorer_tree: Vec::new(),
    is_ready: false,
    is_running: false,
};

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn toggle_file_explorer_node_check(
    app_handle: AppHandle,
    index_path: Vec<usize>,
    state: State<core::state::CoreStateMutex>,
) -> Result<(), ()> {
    println!(
        "Calling command toggle_file_explorer_node_check() with index_path = {:?}.",
        index_path
    );

    let mut core_state_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();

    let mut next_file_explorer = filer::file_explorer::FileExplorer::new(
        core_state_mutable.scanner.file_explorer_tree.to_owned(),
    );
    next_file_explorer.toggle_is_checked(index_path);

    core_state_mutable.scanner.file_explorer_tree = next_file_explorer.into_tree().to_owned();

    app_handle
        .emit_all("scanner:state", &core_state_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn toggle_file_explorer_node_expansion(
    app_handle: AppHandle,
    index_path: Vec<usize>,
    state: State<core::state::CoreStateMutex>,
) -> Result<(), ()> {
    println!(
        "Calling command toggle_file_explorer_node_expansion() with index_path = {:?}.",
        index_path
    );

    let mut core_state_mutex_guard_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();

    let mut next_file_explorer = filer::file_explorer::FileExplorer::new(
        core_state_mutex_guard_mutable
            .scanner
            .file_explorer_tree
            .to_owned(),
    );
    next_file_explorer.toggle_is_expanded(index_path);

    core_state_mutex_guard_mutable.scanner.file_explorer_tree =
        next_file_explorer.into_tree().to_owned();

    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    drop(core_state_mutex_guard_mutable);

    Ok(())
}

// TODO Dry this function, most of it is already in `load_scanner_state()`.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_scanner_state(
    app_handle: AppHandle,
    state: State<'_, core::state::CoreStateMutex>,
) -> Result<(), ()> {
    println!("Calling command get_scanner_state().");

    let core_state_mutex_guard_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();

    println!(
        "core_state_mutex_guard_mutable.scanner: {:?}",
        &core_state_mutex_guard_mutable.scanner
    );

    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    drop(core_state_mutex_guard_mutable);

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn load_scanner_state(
    app_handle: AppHandle,
    state: State<'_, core::state::CoreStateMutex>,
) -> Result<(), ()> {
    println!("Calling command load_scanner_state().");

    let mut core_state_mutex_guard_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();

    if !&core_state_mutex_guard_mutable.scanner.is_ready {
        let file_explorer_tree =
            filer::file_list::list::<String>(false, None, Some(filer::types::FileKind::Directory))
                .into_file_explorer()
                .into_tree();
        let updated_scanner_state = ScannerState {
            file_explorer_tree: file_explorer_tree.to_owned(),
            is_ready: true,
            ..core_state_mutex_guard_mutable.scanner
        };

        core_state_mutex_guard_mutable.scanner = updated_scanner_state;
    }

    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    drop(core_state_mutex_guard_mutable);

    Ok(())
}

// TODO Add and use safe numeric type casting utils: https://stackoverflow.com/a/28280042/2736233.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_scanner(
    app_handle: AppHandle,
    state: State<'_, core::state::CoreStateMutex>,
) -> Result<(), String> {
    println!("Calling command start_scanner().");

    // Set scanner state as "running"
    let mut core_state_mutex_guard_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();
    core_state_mutex_guard_mutable.scanner.is_running = true;
    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    // Send scanner status
    app_handle
        .to_owned()
        .emit_all(
            "scanner:status",
            ScannerStatus {
                current_file_path: "".to_string(),
                progress: 0 as f64,
                step: ScannerStatusStep::Listing,
            },
        )
        .unwrap();

    // List selected paths
    let paths = filer::file_explorer::FileExplorer::new(
        core_state_mutex_guard_mutable
            .scanner
            .file_explorer_tree
            .to_owned(),
    )
    .into_checked_paths();
    println!("Recursively listing files in {:?}.", paths);
    let args: Vec<String> = vec![
        "-rv".to_string(),
        "--follow-dir-symlinks=0".to_string(),
        "--follow-file-symlinks=0".to_string(),
    ]
    .into_iter()
    .chain(paths.to_owned().into_iter())
    .collect();

    // Send scanner status
    app_handle
        .to_owned()
        .emit_all(
            "scanner:status",
            ScannerStatus {
                current_file_path: "".to_string(),
                progress: 0 as f64,
                step: ScannerStatusStep::Counting,
            },
        )
        .unwrap();

    // Recursively count all the non-directory files within the selected paths
    let total_files_length = paths
        .to_owned()
        .into_iter()
        .map(|path| filer::file_list::count(true, Some(path), Some(filer::types::FileKind::File)))
        .sum::<usize>();

    println!("Number of files to scan: {}.", total_files_length);

    drop(core_state_mutex_guard_mutable);

    // Send scanner status
    app_handle
        .to_owned()
        .emit_all(
            "scanner:status",
            ScannerStatus {
                current_file_path: "".to_string(),
                progress: 0 as f64,
                step: ScannerStatusStep::Starting,
            },
        )
        .unwrap();

    libs::cli::run(
        app_handle,
        state.to_owned(),
        String::from("clamscan"),
        args,
        String::from("scanner:status"),
        |log, index| {
            let progress = (index as f64 + f64::from(1)) / total_files_length as f64;
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
                return ScannerStatus {
                    current_file_path: "Done".to_string(),
                    progress,
                    step: ScannerStatusStep::Idle,
                };
            }

            ScannerStatus {
                current_file_path,
                progress,
                step: ScannerStatusStep::Running,
            }
        },
        |log| {
            log.starts_with("Scanning ")
                || log.ends_with(": Empty file")
                || log.ends_with(": Access denied")
        },
    );

    Ok(())
}
