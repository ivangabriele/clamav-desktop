// use crate::print_type_of;

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

/// Checks if a program is installed and globally available in CLI.
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
pub fn is_installed(program: &str) -> bool {
    // https://doc.rust-lang.org/std/env/consts/constant.OS.html
    let which_command = match env::consts::OS {
        "windows" => "where",
        _ => "which",
    };

    let output = Command::new(which_command)
        .arg(program)
        .output()
        .expect(&format!("Failed to run `which {}`.", program));

    output.status.success()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_installed_returns_true_with_an_existing_program() {
        let program = "cargo";

        let result = is_installed(program);

        assert_eq!(result, true);
    }

    #[test]
    fn is_installed_returns_false_with_a_nonexistent_program() {
        let program = "nonexistent-program";

        let result = is_installed(program);

        assert_eq!(result, false);
    }
}

pub fn run(app_handle: AppHandle, binary_path: String, args: Vec<String>, event: String) {
    let app_handle_clone = app_handle.clone();
    let event_as_str = &*event;

    let child = match Command::new(binary_path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
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
        Err(error) => {
            println!("{:?}", error);

            panic!("Bye");
        }
    };

    let mut current_logs: Vec<String> = Vec::new();
    let reader = BufReader::new(stdout);
    reader.lines().filter_map(|line| line.ok()).for_each({
        let current_logs_binding = &mut current_logs;

        move |line| {
            if current_logs_binding.len() == 100 {
                app_handle
                    .emit_all(
                        event_as_str,
                        LogPayload {
                            logs: current_logs_binding.clone(),
                        },
                    )
                    .unwrap();

                current_logs_binding.clear();
            }

            current_logs_binding.push(line);

            println!("{}", current_logs_binding.len());
        }
    });

    app_handle_clone
        .emit_all(
            event_as_str,
            LogPayload {
                logs: current_logs.clone(),
            },
        )
        .unwrap();

    println!("{}", current_logs.len());
}

#[allow(dead_code)]
pub fn run_in_thread(app_handle: AppHandle, binary_path: String, args: Vec<String>, event: String) {
    let join_handle = thread::spawn(move || {
        let app_handle_clone = app_handle.clone();
        let event_as_str = &*event;

        let child = match Command::new(binary_path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
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
            Err(error) => {
                println!("{:?}", error);

                panic!("Bye");
            }
        };

        let mut current_logs: Vec<String> = Vec::new();
        let reader = BufReader::new(stdout);
        reader.lines().filter_map(|line| line.ok()).for_each({
            let current_logs_binding = &mut current_logs;

            move |line| {
                // println!("{}", line);

                if current_logs_binding.len() == 100 {
                    app_handle
                        .emit_all(
                            event_as_str,
                            LogPayload {
                                logs: current_logs_binding.clone(),
                            },
                        )
                        .unwrap();

                    current_logs_binding.clear();
                }

                current_logs_binding.push(line);

                println!("{}", current_logs_binding.len());
            }
        });

        app_handle_clone
            .emit_all(
                event_as_str,
                LogPayload {
                    logs: current_logs.clone(),
                },
            )
            .unwrap();

        println!("{}", current_logs.len());
    });

    join_handle.join().unwrap();
}
