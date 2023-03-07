// We allow `unused_imports` & `unused_variables` here because the `debug_assertions` are unused in production
// but required for development.

#[allow(unused_imports)]
use tauri::{LogicalSize, Manager};

mod commands;
mod libs;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("Hello Clamav Desktop!");

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
        .invoke_handler(tauri::generate_handler![commands::start_scan])
        .run(tauri::generate_context!())
        .expect("An error happened while running Tauri application.");
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
