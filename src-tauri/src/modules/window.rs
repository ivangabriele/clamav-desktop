use tauri::{AppHandle, Manager};

#[cfg(not(tarpaulin_include))]
pub fn toggle(app_handle: &AppHandle) -> () {
    match app_handle.get_window("main") {
        Some(window) => {
            let item_handle = app_handle.tray_handle().get_item("toggle");

            if window.is_visible().unwrap() {
                window.hide().unwrap();

                item_handle.set_title("Show").unwrap();
            } else {
                window.show().unwrap();

                item_handle.set_title("Hide").unwrap();
            }
        }
        None => {}
    }
}
