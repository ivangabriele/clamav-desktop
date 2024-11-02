use tauri::{
    menu::{self, Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

use crate::globals;

use super::window;

#[cfg(not(tarpaulin_include))]
pub fn new_tray_icon<R: Runtime>(app_handle: &AppHandle<R>) -> tauri::Result<()> {
    let menu = new_menu(app_handle, false);

    let _ = TrayIconBuilder::with_id(globals::MAIN_TRAY_ICON_ID)
        .icon(app_handle.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app_handle, event| match event.id.as_ref() {
            "show" => window::show(app_handle),
            "hide" => window::hide(app_handle),
            "quit" => app_handle.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let window = tray
                    .app_handle()
                    .get_window(globals::MAIN_TRAY_ICON_ID)
                    .expect("Could not get window.");

                let _ = window.show();
                let _ = window.set_focus();
            }
        })
        .build(app_handle);

    Ok(())
}

pub fn new_menu<R: Runtime>(app_handle: &tauri::AppHandle<R>, is_windows_hidden: bool) -> menu::Menu<R> {
    let toggle_menu_item = match is_windows_hidden {
        true => MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>).unwrap(),
        false => MenuItem::with_id(app_handle, "hide", "Hide", true, None::<&str>).unwrap(),
    };
    let quit_menu_item = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>).unwrap();

    Menu::with_items(app_handle, &[&toggle_menu_item, &quit_menu_item]).unwrap()
}
