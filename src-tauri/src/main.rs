// We allow `unused_imports` & `unused_variables` here
// because the `debug_assertions` are unused in production but required for development.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(unused_imports)]
use tauri::LogicalSize;
use tauri::{api, Manager, SystemTrayEvent};

mod cloud;
mod copilot;
mod dashboard;
mod globals;
mod libs;
mod scanner;
mod settings;
mod system;

#[cfg(not(tarpaulin_include))]
fn main() {
    let context = tauri::generate_context!();
    let system_tray = system::tray::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(
            #[allow(unused_variables)]
            |app: &mut tauri::App| {
                let app_handle = app.handle();
                tauri::async_runtime::block_on(async move {
                    #[cfg(debug_assertions)]
                    {
                        let window = app_handle.get_window("main").expect("Could not get window.");
                        window
                            .set_size(LogicalSize::<u32> {
                                height: 900,
                                width: 1024,
                            })
                            .expect("Could not set window size.");
                        window.set_always_on_top(false).expect("Could not set always on top.");

                        window.open_devtools();
                    }

                    // Store config in a variable to extend its lifetime
                    let config_binding = app_handle.config();
                    let config = config_binding.as_ref();

                    // let app_cache_dir = api::path::app_cache_dir(config).expect("Could not get cache directory.");
                    // println!("Cache directory: {:?}", app_cache_dir);
                    // let app_config_dir = api::path::app_config_dir(config).expect("Could not get config directory.");
                    // println!("Config directory: {:?}", app_config_dir);
                    // let app_data_dir = api::path::app_data_dir(config).expect("Could not get data directory.");
                    // println!("Data directory: {:?}", app_data_dir);
                    // let app_local_data_dir =
                    //     api::path::app_local_data_dir(config).expect("Could not get local data directory.");
                    // println!("Local data directory: {:?}", app_local_data_dir);
                    // let app_log_dir = api::path::app_log_dir(config).expect("Could not get log directory.");
                    // println!("Log directory: {:?}", app_log_dir);

                    let mut config_directory_path = globals::CONFIG_DIRECTORY_PATH.lock().await;
                    *config_directory_path =
                        api::path::app_config_dir(config).expect("Could not get app config directory path.");
                    let mut local_data_directory_path = globals::LOCAL_DATA_DIRECTORY_PATH.lock().await;
                    *local_data_directory_path =
                        api::path::app_local_data_dir(config).expect("Could not get local data directory path.");
                    let mut log_directory_path = globals::LOG_DIRECTORY_PATH.lock().await;
                    *log_directory_path =
                        api::path::app_log_dir(config).expect("Could not get app log directory path.");

                    debug!("main()", "App started.");
                });

                Ok(())
            },
        )
        // https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs
        .manage(cloud::state::CloudSharedState(Default::default()))
        .manage(copilot::state::CopilotSharedState(Default::default()))
        .manage(dashboard::state::DashboardSharedState(Default::default()))
        .manage(scanner::state::SharedScannerState(Default::default()))
        .manage(settings::state::SharedSettingsState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            cloud::commands::get_cloud_state,
            cloud::commands::start_cloud_daemon,
            cloud::commands::start_cloud_update,
            cloud::commands::stop_cloud_daemon,
            copilot::commands::get_copilot_state,
            copilot::commands::start_copilot_checklist,
            dashboard::commands::get_dashboard_state,
            dashboard::commands::start_daemon,
            dashboard::commands::stop_daemon,
            scanner::commands::get_scanner_state,
            scanner::commands::load_scanner_state,
            scanner::commands::start_scanner,
            scanner::commands::stop_scanner,
            scanner::commands::toggle_file_explorer_node_check,
            scanner::commands::toggle_file_explorer_node_expansion,
            settings::commands::get_settings_state,
            settings::commands::load_settings_state,
            settings::commands::update_clamd_conf_file_source,
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app_handle, event| match event {
            SystemTrayEvent::LeftClick {
                position: _, size: _, ..
            } => system::window::toggle(app_handle),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "toggle" => system::window::toggle(app_handle),
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
                system::window::toggle(&app_handle);
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
