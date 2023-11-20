// We allow `unused_imports` & `unused_variables` here
// because the `debug_assertions` are unused in production but required for development.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(unused_imports)]
use tauri::LogicalSize;
use tauri::{api, Manager, SystemTrayEvent};

mod cloud;
mod daemon;
mod globals;
mod libs;
mod modules;
mod scanner;

#[cfg(not(tarpaulin_include))]
fn main() {
    let context = tauri::generate_context!();

    let system_tray = modules::tray::new();

    tauri::Builder::default()
        .setup(
            #[allow(unused_variables)]
            |app: &mut tauri::App| {
                #[cfg(debug_assertions)]
                {
                    let window = app.get_window("main").expect("Could not get window.");
                    window
                        .set_size(LogicalSize::<u32> {
                            height: 768,
                            width: 1024,
                        })
                        .expect("Could not set window size.");
                    window
                        .set_always_on_top(true)
                        .expect("Could not set always on top.");

                    window.open_devtools();
                }

                // Store config in a variable to extend its lifetime
                let config_binding = app.config();
                let config = config_binding.as_ref();

                let mut log_directory_path = globals::LOG_DIRECTORY_PATH
                    .lock()
                    .expect("Could not lock log directory path.");
                *log_directory_path =
                    api::path::app_log_dir(config).expect("Could not get log directory.");

                Ok(())
            },
        )
        // https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs
        .manage(cloud::CloudStateArcMutex(Default::default()))
        .manage(daemon::DaemonStateArcMutex(Default::default()))
        .manage(scanner::state::SharedScannerState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            cloud::get_cloud_state,
            cloud::start_cloud_daemon,
            cloud::start_cloud_update,
            cloud::stop_cloud_daemon,
            daemon::get_daemon_state,
            daemon::start_daemon,
            daemon::stop_daemon,
            scanner::commands::get_scanner_state,
            scanner::commands::load_scanner_state,
            scanner::commands::start_scanner,
            scanner::commands::stop_scanner,
            scanner::commands::toggle_file_explorer_node_check,
            scanner::commands::toggle_file_explorer_node_expansion,
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app_handle, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => modules::window::toggle(app_handle),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "toggle" => modules::window::toggle(app_handle),
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();

                let app_handle = event.window().app_handle();
                modules::window::toggle(&app_handle);
            }
            _ => {}
        })
        .build(context)
        .expect("An error happened while building ClamAV Desktop.")
        .run(|app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                println!("Exit requested!!!");

                api.prevent_exit();

                let item_handle = app_handle.tray_handle().get_item("toggle");
                item_handle.set_title("Show").unwrap();
            }
            _ => {}
        });
}
