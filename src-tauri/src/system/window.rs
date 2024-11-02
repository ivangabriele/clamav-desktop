use tauri::{AppHandle, Manager, Runtime};

use crate::globals;

use super::tray;

// #[cfg(not(tarpaulin_include))]
pub fn hide<R: Runtime>(app_handle: &AppHandle<R>) -> () {
    let window = app_handle
        .get_window(globals::MAIN_WINDOW_LABEL)
        .expect("Could not get window.");

    let _ = window.hide();

    if let Some(tray_icon) = app_handle.tray_by_id(globals::MAIN_TRAY_ICON_ID) {
        let next_menu = tray::new_menu(app_handle, true);

        let _ = tray_icon.set_menu(Some(next_menu));
    }
}

// #[cfg(not(tarpaulin_include))]
pub fn show<R: Runtime>(app_handle: &AppHandle<R>) -> () {
    let window = app_handle
        .get_window(globals::MAIN_WINDOW_LABEL)
        .expect("Could not get window.");

    let _ = window.show();
    let _ = window.set_focus();

    if let Some(tray_icon) = app_handle.tray_by_id(globals::MAIN_TRAY_ICON_ID) {
        let next_menu = tray::new_menu(app_handle, false);

        let _ = tray_icon.set_menu(Some(next_menu));
    }
}
