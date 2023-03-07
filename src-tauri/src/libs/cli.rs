use serde::Serialize;
use std::{
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};
use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    logs: Vec<String>,
}

/// Checks if the provided program name is installed and globally available in CLI.
///
/// # Examples
///
/// ```
/// use crate::libs::cli;
///
/// let is_cd_installed = cli::is_installed("cd");
///
/// assert_eq!(true, is_cd_installed);
/// ```
#[allow(dead_code)]
pub fn is_installed(program_name: &str) -> bool {
    // https://doc.rust-lang.org/std/env/consts/constant.OS.html
    let which_command = match env::consts::OS {
        "windows" => "where",
        _ => "which",
    };

    let output = Command::new(which_command)
        .arg(program_name)
        .output()
        .expect(&format!("Failed to run `which {}`.", program_name));

    output.status.success()
}

pub fn run<B, F, P>(
    app_handle: AppHandle,
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
                        .emit_all(
                            &*event_name,
                            event_payload_builder(line.to_owned(), log_index),
                        )
                        .unwrap();

                    log_index += 1;
                }
            }
        });
}

#[allow(dead_code)]
pub fn run_in_thread<B, F, P>(
    app_handle: AppHandle,
    binary_path: String,
    args: Vec<String>,
    event_name: String,
    event_payload_builder: B,
    filter_log: F,
) -> ()
where
    B: Fn(String, usize) -> P,
    B: Send + 'static,
    F: Fn(String) -> bool,
    F: Send + 'static,
    P: Serialize + Clone,
{
    let join_handle = thread::spawn(move || {
        run(
            app_handle,
            binary_path,
            args,
            event_name,
            event_payload_builder,
            filter_log,
        );
    });

    join_handle.join().unwrap();
}
