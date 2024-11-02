use tauri::LogicalSize;
use tauri::Manager;

mod cloud;
mod copilot;
mod dashboard;
mod globals;
mod libs;
mod scanner;
mod settings;
mod system;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
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
        .setup(
            #[allow(unused_variables)]
            |app: &mut tauri::App| {
                let app_handle = app.handle();
                tauri::async_runtime::block_on(async move {
                    #[cfg(all(desktop))]
                    {
                        system::tray::new_tray_icon(app_handle).expect("Could not create system tray.");
                    }

                    #[cfg(debug_assertions)]
                    {
                        let window = app_handle.get_window(globals::MAIN_WINDOW_LABEL).expect(
                            format!("Could not get window with label `{}`.", globals::MAIN_WINDOW_LABEL).as_str(),
                        );

                        window
                            .set_size(LogicalSize::<u32> {
                                height: 900,
                                width: 1024,
                            })
                            .expect("Could not set window size.");
                        window.set_always_on_top(false).expect("Could not set always on top.");
                        window.webviews().first().unwrap().open_devtools();
                    }

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
                    *config_directory_path = app_handle
                        .path()
                        .app_config_dir()
                        .expect("Could not get app config directory path.");
                    let mut local_data_directory_path = globals::LOCAL_DATA_DIRECTORY_PATH.lock().await;
                    *local_data_directory_path = app_handle
                        .path()
                        .app_local_data_dir()
                        .expect("Could not get local data directory path.");
                    let mut log_directory_path = globals::LOG_DIRECTORY_PATH.lock().await;
                    *log_directory_path = app_handle
                        .path()
                        .app_log_dir()
                        .expect("Could not get app log directory path.");

                    debug!("main()", "App started.");
                });

                Ok(())
            },
        )
        .build(context)
        .expect("An error happened while building ClamAV Desktop.")
        .run(|app_handle, run_event| match run_event {
            tauri::RunEvent::WindowEvent {
                event: window_event, ..
            } => match window_event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();

                    system::window::hide(app_handle);
                }
                _ => {}
            },
            _ => {}
        });
}
