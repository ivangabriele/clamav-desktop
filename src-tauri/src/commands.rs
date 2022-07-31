// Tauri executes async commands on a separate thread using `async_runtime::spawn`.
// https://tauri.app/v1/guides/features/command/#async-commands

use tauri::AppHandle;

use crate::libs::cli;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub async fn find(app_handle: AppHandle, path: String) -> Result<(), String> {
    println!("Start find in {}.", path);

    let binary_path = String::from("find");
    let event = String::from("find:log");

    cli::run(app_handle, binary_path, vec![path], event);

    Ok(())
}
