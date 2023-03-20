use serde::Serialize;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use tauri::{AppHandle, Manager, State};

use crate::core;

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    logs: Vec<String>,
}

// Because this will be replaced by local cli lib:
#[cfg(not(tarpaulin_include))]
pub fn run<B, F, P>(
    app_handle: AppHandle,
    state: State<'_, core::state::CoreStateMutex>,
    binary_path: String,
    args: Vec<String>,
    event_name: String,
    event_payload_builder: B,
    filter_log: F,
) -> ()
where
    B: Fn(String, usize) -> P,
    F: Fn(String) -> bool,
    P: Serialize + Clone,
{
    let app_handle_owner = app_handle.to_owned();

    let child = match Command::new(binary_path)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        // TODO Properly handle this error.
        Err(error) => {
            println!("{:?}", error);

            panic!("Bye");
        }
    };

    let stdout = match child
        .stdout
        .ok_or_else(|| println!("Could not capture standard output."))
    {
        Ok(stdout) => stdout,
        // TODO Properly handle this error.
        Err(error) => {
            println!("{:?}", error);

            panic!("Bye");
        }
    };

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

    let mut core_state_mutable = state
        .0
        .lock()
        // TODO Properly handle errors here.
        .unwrap_or_else(|error| panic!("{:?}", &error));
    core_state_mutable.scanner.is_running = false;
    println!("{:?}", core_state_mutable.scanner);
    app_handle_owner
        .emit_all("scanner:state", &core_state_mutable.scanner)
        // TODO Properly handle errors here.
        .unwrap();
}

// #[allow(dead_code)]
// pub fn run_in_thread<B, F, P>(
//     app_handle: AppHandle,
//     state: State<'_, core::state::CoreStateMutex>,
//     binary_path: String,
//     args: Vec<String>,
//     event_name: String,
//     event_payload_builder: B,
//     filter_log: F,
// ) -> ()
// where
//     B: Fn(String, usize) -> P,
//     B: Send + 'static,
//     F: Fn(String) -> bool,
//     F: Send + 'static,
//     P: Serialize + Clone,
// {
//     let join_handle = thread::spawn(move || {
//         run(
//             app_handle,
//             state,
//             binary_path,
//             args,
//             event_name,
//             event_payload_builder,
//             filter_log,
//         );
//     });

//     join_handle.join().unwrap();
// }
