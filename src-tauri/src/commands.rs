// Tauri executes async commands on a separate thread using `async_runtime::spawn`.
// https://tauri.app/v1/guides/features/command/#async-commands

use tauri::AppHandle;

use crate::libs::{cli, file};

#[derive(Clone, serde::Serialize)]
struct EventPayloadForScanStatus {
    current_file_path: String,
    progress: f64,
}

// TODO Add and use safe numeric type casting utils: https://stackoverflow.com/a/28280042/2736233.
#[tauri::command]
pub async fn start_scan(
    app_handle: AppHandle,
    directory_absolute_path: String,
) -> Result<(), String> {
    println!("Start scan in {}.", directory_absolute_path);

    let file_absolute_paths = file::list(
        directory_absolute_path.to_owned(),
        file::FileType::File,
        true,
    );
    let file_absolute_paths_length = file_absolute_paths.len() as f64;

    cli::run(
        app_handle,
        String::from("clamscan"),
        vec![String::from("-rv"), directory_absolute_path],
        String::from("scan:status"),
        |log, index| {
            let index_as_f64 = index as f64;
            let progress = (index_as_f64 + f64::from(1)) / file_absolute_paths_length;
            // TODO Find a better way to extract the path (maybe via a regex).
            let current_file_path = &*log[9..].to_owned();
            let current_file_path_as_string = current_file_path.to_string();

            EventPayloadForScanStatus {
                current_file_path: current_file_path_as_string,
                progress,
            }
        },
        |log| log.starts_with("Scanning "),
    );

    Ok(())
}
