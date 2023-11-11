use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use crate::{core, libs};
use filer;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ScannerStatusStep {
    /// Counting the files to scan.
    Counting,
    /// Default step (= waiting for a new job).
    #[default]
    Idle,
    /// Listing the files to scan.
    Listing,
    /// Scanning the files.
    Running,
    /// Starting (= has called `clamscan` CLI command).
    Starting,
}

#[derive(Clone, Debug, Default, Serialize)]
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

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn toggle_file_explorer_node_check(
    app_handle: AppHandle,
    index_path: Vec<usize>,
    state: State<core::state::SharedCoreState>,
) -> Result<(), ()> {
    println!(
        "Calling command toggle_file_explorer_node_check() with index_path = {:?}.",
        index_path
    );

    let mut core_state_mutex_guard = state.0.lock().unwrap();

    let mut next_file_explorer = filer::file_explorer::FileExplorer::new(
        core_state_mutex_guard.scanner.file_explorer_tree.to_owned(),
    );
    next_file_explorer.toggle_is_checked(index_path);

    core_state_mutex_guard.scanner.file_explorer_tree = next_file_explorer.into_tree().to_owned();

    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard.scanner)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub fn toggle_file_explorer_node_expansion(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
    index_path: Vec<usize>,
) -> Result<(), ()> {
    println!(
        "Calling command toggle_file_explorer_node_expansion() with index_path = {:?}.",
        index_path
    );

    let mut core_state_mutex_guard = state.0.lock().unwrap();

    let mut next_file_explorer = filer::file_explorer::FileExplorer::new(
        core_state_mutex_guard.scanner.file_explorer_tree.to_owned(),
    );
    next_file_explorer.toggle_is_expanded(index_path);

    core_state_mutex_guard.scanner.file_explorer_tree = next_file_explorer.into_tree().to_owned();

    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard.scanner)
        .unwrap();

    Ok(())
}

// TODO Dry this function, most of it is already in `load_scanner_state()`.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_scanner_state(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    println!("Calling command get_scanner_state().");

    let core_state_mutex_guard = state.0.lock().unwrap();

    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard.scanner)
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn load_scanner_state(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    println!("Calling command load_scanner_state().");

    let mut core_state_mutex_guard = state.0.lock().unwrap();

    if !&core_state_mutex_guard.scanner.is_ready {
        let file_explorer_tree =
            filer::file_list::list::<String>(false, None, Some(filer::types::FileKind::Directory))
                .into_file_explorer()
                .into_tree();
        let updated_scanner_state = ScannerState {
            file_explorer_tree: file_explorer_tree.to_owned(),
            is_ready: true,
            ..core_state_mutex_guard.scanner
        };

        core_state_mutex_guard.scanner = updated_scanner_state;
    }

    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard.scanner)
        .unwrap();

    Ok(())
}

// TODO Add and use safe numeric type casting utils: https://stackoverflow.com/a/28280042/2736233.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_scanner(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    println!("Calling command start_scanner().");

    let mut core_state_mutex_guard = state.0.lock().unwrap();

    let file_explorer_tree = core_state_mutex_guard.scanner.file_explorer_tree.clone();

    // Update scanner state
    core_state_mutex_guard.scanner.is_running = true;
    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard.scanner)
        .unwrap();

    // Update scanner status
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
    let paths = filer::file_explorer::FileExplorer::new(file_explorer_tree).into_checked_paths();
    println!("Recursively listing files in {:?}.", paths);
    let args: Vec<String> = vec![
        "-rv".to_string(),
        "--follow-dir-symlinks=0".to_string(),
        "--follow-file-symlinks=0".to_string(),
    ]
    .into_iter()
    .chain(paths.to_owned().into_iter())
    .collect();

    // Update scanner status
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

    // Update scanner status
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

    let child = libs::cli::run(
        app_handle,
        String::from("clamscan"),
        args,
        String::from("scanner:status"),
        move |log, index| {
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

    core_state_mutex_guard.scanner_thread = Some(child);

    Ok(())
}

#[tauri::command]
pub fn stop_scanner(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    println!("Calling command stop_scanner().");

    let mut core_state_mutex_guard = state.0.lock().unwrap();
    if core_state_mutex_guard.scanner_thread.is_none() {
        return Ok(());
    }

    let child_mutant = core_state_mutex_guard.scanner_thread.as_mut().unwrap();
    child_mutant
        .kill()
        // .map_err(|_| "Failed to kill scanner process.")
        .unwrap();

    // Update scanner state
    core_state_mutex_guard.scanner.is_running = false;
    app_handle
        .emit_all("scanner:state", &core_state_mutex_guard.scanner)
        .unwrap();

    // Update scanner status
    app_handle
        .to_owned()
        .emit_all(
            "scanner:status",
            ScannerStatus {
                current_file_path: "".to_string(),
                progress: 0 as f64,
                step: ScannerStatusStep::Idle,
            },
        )
        .unwrap();

    Ok(())
}
