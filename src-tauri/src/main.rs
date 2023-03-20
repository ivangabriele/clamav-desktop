// We allow `unused_imports` & `unused_variables` here because the `debug_assertions` are unused in production
// but required for development.

use std::sync::Mutex;

#[allow(unused_imports)]
use tauri::{LogicalSize, Manager};

mod core;
mod libs;
mod scanner;

#[cfg(not(tarpaulin_include))]
fn main() {
    let initial_core_state = core::state::CoreState {
        scanner: scanner::INITIAL_SCANNER_STATE,
    };

    tauri::Builder::default()
        .setup(
            #[allow(unused_variables)]
            |app| {
                #[cfg(debug_assertions)]
                {
                    let window = app.get_window("ClamAV").unwrap();
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
        .manage(core::state::CoreStateMutex(Mutex::new(initial_core_state)))
        .invoke_handler(tauri::generate_handler![
            scanner::get_scanner_state,
            scanner::load_scanner_state,
            scanner::start_scanner,
            scanner::toggle_file_explorer_node_check,
            scanner::toggle_file_explorer_node_expansion
        ])
        .run(tauri::generate_context!())
        // TODO Properly handle errors here.
        .expect("An error happened during ClamAV Desktop boot.");
}

// fn run() {
//     let _os = OS;
//     let output = Command::new("clamscanz")
//         .arg("--version")
//         .output()
//         .expect("Failed to get clamscan version.");

//     let println!("status: {}", output.status);
//     print_type_of(&output.status);
//     println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
// }
