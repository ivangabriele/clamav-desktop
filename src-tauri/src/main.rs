// We allow `unused_imports` & `unused_variables` here
// because the `debug_assertions` are unused in production but required for development.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(unused_imports)]
use tauri::LogicalSize;
use tauri::{Manager, SystemTrayEvent};

mod cloud;
mod core;
mod daemon;
mod libs;
mod modules;
mod scanner;

#[cfg(not(tarpaulin_include))]
fn main() {
    let system_tray = modules::tray::new();

    tauri::Builder::default()
        .setup(
            #[allow(unused_variables)]
            |app| {
                #[cfg(debug_assertions)]
                {
                    let window = app.get_window("main").unwrap();
                    window
                        .set_size(LogicalSize::<u32> {
                            height: 768,
                            width: 1024,
                        })
                        .unwrap();
                    window.set_always_on_top(true).unwrap();

                    window.open_devtools();
                }

                Ok(())
            },
        )
        // https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs
        .manage(core::state::SharedCoreState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            // cloud::start_update,
            // cloud::stop_update,
            cloud::get_cloud_state,
            daemon::get_daemon_state,
            daemon::start_daemon,
            daemon::stop_daemon,
            scanner::get_scanner_state,
            scanner::load_scanner_state,
            scanner::start_scanner,
            scanner::stop_scanner,
            scanner::toggle_file_explorer_node_check,
            scanner::toggle_file_explorer_node_expansion,
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
        .build(tauri::generate_context!())
        // TODO Properly handle errors here.
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
