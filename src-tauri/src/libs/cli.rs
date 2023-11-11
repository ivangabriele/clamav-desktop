use serde::Serialize;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};
use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    logs: Vec<String>,
}

// Because this will be replaced by local cli lib:
#[cfg(not(tarpaulin_include))]
pub fn run<B, F, P>(
    app_handle: AppHandle,
    binary_path: String,
    args: Vec<String>,
    event_name: String,
    event_payload_builder: B,
    filter_log: F,
) -> Child
where
    B: Fn(String, usize) -> P + 'static + std::marker::Send,
    F: Fn(String) -> bool + 'static + std::marker::Send,
    P: Serialize + Clone + 'static + std::marker::Send,
{
    // let app_handle_owned = app_handle.to_owned();

    let mut child = Command::new(binary_path)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process.");

    let stdout = child
        .stdout
        .take()
        .expect("Failed to attach standard output.");

    std::thread::spawn(move || {
        let mut log_index = 0;
        let reader = BufReader::new(stdout);
        reader
            .lines()
            // TODO Is it the best way to achieve that?
            .filter_map(|line| line.ok())
            .for_each({
                move |line| {
                    #[cfg(debug_assertions)]
                    {
                        println!("[libs::cli::run()] {}", line);
                    }

                    if filter_log(line.to_owned()) {
                        app_handle
                            .to_owned()
                            .emit_all(
                                &*event_name,
                                event_payload_builder(line.to_owned(), log_index),
                            )
                            .unwrap();

                        log_index += 1;
                    }
                }
            });

        // let mut core_state_mutex_guard = app_handle_owned
        //     .state::<core::state::SharedCoreState>()
        //     .inner()
        //     .0
        //     .lock()
        //     .unwrap();
    });

    child
}
