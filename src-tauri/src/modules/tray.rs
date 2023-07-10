use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

#[cfg(not(tarpaulin_include))]
pub fn new() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    let system_tray = SystemTray::new().with_menu(tray_menu);

    system_tray
}
