use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Manager, State};

use crate::{core, libs};
use filer;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ScannerStatusStep {
    /// Counting files to scan.
    Counting,
    /// Default step (= waiting for a new job).
    #[default]
    Idle,
    /// Listing files to scan.
    Listing,
    /// Scanning files.
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
    use std::env;

    println!("Calling command start_scanner().");

    let is_dev_mode = env::var("TAURI_DEV").is_ok();
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
        // "clamscan".to_string(),
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

    let mut args_with_clamscan = vec!["clamscan".to_string()];
    args_with_clamscan.extend(args.iter().cloned());
    let child = if is_dev_mode {
        libs::cli::run(String::from("pkexec"), args_with_clamscan)
    } else {
        libs::cli::run(String::from("clamscan"), args)
    };

    let shared_child = Arc::new(Mutex::new(Some(child)));
    let shared_child_clone = shared_child.clone();

    let app_handle_clone_for_log = app_handle.clone();
    let app_handle_clone_for_exit = app_handle.clone();
    thread::spawn(move || {
        let mut child = shared_child_clone
            .lock()
            .unwrap()
            .take()
            .expect("Child process handle was taken");

        let stdout = child
            .stdout
            .take()
            .expect("Failed to attach standard output.");

        let mut log_index = 0;
        let reader = BufReader::new(stdout);
        reader
            .lines()
            // TODO Is it the best way to achieve that?
            .filter_map(|line| line.ok())
            .for_each({
                move |line| {
                    #[cfg(debug_assertions)]
                    {
                        println!("[libs::cli::run()] {}", line);
                    }

                    if filter_log(line.to_owned()) {
                        let next_status =
                            get_status_from_log(line.to_owned(), log_index, total_files_length);
                        app_handle_clone_for_log
                            .emit_all("scanner:status", next_status)
                            .unwrap();
                    }

                    log_index += 1;
                }
            });

        let _ = child.wait().expect("Failed to wait for child exit.");

        // Update scanner state
        let mut core_state_mutex_guard = app_handle_clone_for_exit
            .state::<core::state::SharedCoreState>()
            .inner()
            .0
            .lock()
            .unwrap();
        core_state_mutex_guard.scanner.is_running = false;
        app_handle_clone_for_exit
            .emit_all("scanner:state", &core_state_mutex_guard.scanner)
            .unwrap();
    });

    let mut core_state_mutex_guard = app_handle
        .state::<core::state::SharedCoreState>()
        .inner()
        .0
        .lock()
        .unwrap();
    core_state_mutex_guard.scanner_thread = Some(shared_child);

    Ok(())
}

#[tauri::command]
pub fn stop_scanner(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    println!("Calling command stop_scanner().");

    let core_state_mutex_guard = state.0.lock().unwrap();
    // if core_state_mutex_guard.scanner_thread.is_none() {
    //     return Ok(());
    // }

    if let Some(child_arc) = core_state_mutex_guard.scanner_thread.as_ref() {
        let mut child_option = child_arc.lock().unwrap();

        if let Some(child) = child_option.as_mut() {
            child.kill().expect("Failed to kill scanner process.");
            *child_option = None;
        }

        // Update scanner state
        let mut core_state_mutex_guard = app_handle
            .state::<core::state::SharedCoreState>()
            .inner()
            .0
            .lock()
            .unwrap();
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
    }

    Ok(())
}

fn filter_log(log: String) -> bool {
    log.starts_with("Scanning ")
        || log.ends_with(": Empty file")
        || log.ends_with(": Access denied")
}

fn get_status_from_log(log: String, log_index: usize, total_files_length: usize) -> ScannerStatus {
    let progress = (log_index as f64 + f64::from(1)) / total_files_length as f64;
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
}
