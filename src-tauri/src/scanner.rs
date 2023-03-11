use tauri::{AppHandle, Manager, State};

use crate::{core, libs};
// use cli;
use filer;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScannerState {
    drives: Vec<String>,
    file_explorer_tree: filer::FileExplorerTree,
    is_ready: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ScannerStatus {
    current_file_path: String,
    progress: f64,
}

pub const INITIAL_SCANNER_STATE: ScannerState = ScannerState {
    drives: Vec::new(),
    file_explorer_tree: Vec::new(),
    is_ready: false,
};

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

    let mut next_file_explorer =
        filer::FileExplorer::new(core_state_mutable.scanner.file_explorer_tree.to_owned());
    next_file_explorer.toggle_is_checked(index_path);

    core_state_mutable.scanner.file_explorer_tree = next_file_explorer.into_tree().to_owned();

    app_handle
        .emit_all("scanner:state", &core_state_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    Ok(())
}

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

    let mut core_state_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();

    let mut next_file_explorer =
        filer::FileExplorer::new(core_state_mutable.scanner.file_explorer_tree.to_owned());
    next_file_explorer.toggle_is_expanded(index_path);

    core_state_mutable.scanner.file_explorer_tree = next_file_explorer.into_tree().to_owned();

    app_handle
        .emit_all("scanner:state", &core_state_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    Ok(())
}

// TODO Dry this function, most of it is already in `load_scanner_state()`.
#[tauri::command]
pub async fn get_scanner_state(
    app_handle: AppHandle,
    state: State<'_, core::state::CoreStateMutex>,
) -> Result<(), ()> {
    println!("Calling command get_scanner_state().");

    let mut core_state_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();

    if !&core_state_mutable.scanner.is_ready {
        let drives = filer::drive::list();
        let file_explorer_tree = filer::list(&drives[0], false, Some(filer::FileKind::Directory))
            .into_file_explorer()
            .into_tree();
        let updated_scanner_state = ScannerState {
            drives: drives.to_owned(),
            file_explorer_tree: file_explorer_tree.to_owned(),
            is_ready: true,
        };

        println!("{:?}", &updated_scanner_state);

        core_state_mutable.scanner = updated_scanner_state;

        app_handle
            .emit_all("scanner:state", &core_state_mutable.scanner)
            // TODO Properly handle errors here.
            .unwrap();

        return Ok(());
    }

    app_handle
        .emit_all("scanner:state", &core_state_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn load_scanner_state(
    app_handle: AppHandle,
    state: State<'_, core::state::CoreStateMutex>,
) -> Result<(), ()> {
    println!("Calling command load_scanner_state().");

    let mut core_state_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap();

    let drives = filer::drive::list();
    let file_explorer_tree = filer::list(&drives[0], false, Some(filer::FileKind::Directory))
        .into_file_explorer()
        .into_tree();
    let updated_scanner_state = ScannerState {
        drives: drives.to_owned(),
        file_explorer_tree: file_explorer_tree.to_owned(),
        is_ready: true,
    };

    println!("{:?}", &updated_scanner_state);

    core_state_mutable.scanner = updated_scanner_state;

    app_handle
        .emit_all("scanner:state", &core_state_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();

    Ok(())
}

// TODO Add and use safe numeric type casting utils: https://stackoverflow.com/a/28280042/2736233.
#[tauri::command]
pub async fn start_scanner(
    app_handle: AppHandle,
    directory_absolute_path: String,
) -> Result<(), String> {
    println!(
        "Calling command start_scan() with directory_absolute_path = {}.",
        directory_absolute_path
    );

    let file_list = filer::list(
        directory_absolute_path.to_owned(),
        true,
        Some(filer::FileKind::File),
    )
    .into_strings();
    let file_list_length = file_list.len() as f64;

    libs::cli::run(
        app_handle,
        String::from("clamscan"),
        vec![String::from("-rv"), directory_absolute_path],
        String::from("scanner:status"),
        |log, index| {
            let index_as_f64 = index as f64;
            let progress = (index_as_f64 + f64::from(1)) / file_list_length;
            // TODO Find a better way to extract the path (maybe via a regex).
            // This actually removes the "Scanning " part at the start of each log line
            let current_file_path = &*log[9..].to_owned();
            let current_file_path_as_string = current_file_path.to_string();

            ScannerStatus {
                current_file_path: current_file_path_as_string,
                progress,
            }
        },
        |log| log.starts_with("Scanning "),
    );

    Ok(())
}
