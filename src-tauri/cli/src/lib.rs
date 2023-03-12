use std::{
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

use common;
use error::CliError;

mod error;

pub fn exec(command: String, args: Vec<String>) -> Result<String, CliError> {
    match Command::new(command.to_owned())
        .args(args.to_owned())
        .output()
    {
        Ok(output) => {
            let stdout_as_string = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr_as_string = String::from_utf8_lossy(&output.stderr).trim().to_string();

            if stdout_as_string.is_empty() {
                return Err(CliError::new(
                    format!(
                        "`Command.output()` failed with command {} and args {:?}.",
                        command, args
                    ),
                    Box::new(common::CommonError::new(stderr_as_string)),
                ));
            }

            return Ok(stdout_as_string);
        }
        Err(error) => Err(CliError::new(
            format!(
                "`Command.output()` failed with command {} and args {:?}.",
                command, args
            ),
            Box::new(error),
        )),
    }
}

/// Checks if the provided program name is installed and globally available in CLI.
///
/// # Examples
///
/// ```
/// use cli;
///
/// let is_cargo_installed = cli::is_installed("cargo".to_string());
///
/// assert_eq!(true, is_cargo_installed);
/// ```
pub fn is_installed(program_name: String) -> bool {
    // https://doc.rust-lang.org/std/env/consts/constant.OS.html
    let command = match env::consts::OS {
        "windows" => "where",
        _ => "which",
    };
    let args = Vec::from(&[program_name][..]);

    exec(command.to_string(), args).is_ok()
}

pub fn run<C>(command: String, args: Vec<String>, callback_option: Option<C>) -> ()
where
    C: Fn(usize, String) -> (),
    C: Send + 'static,
{
    let child = match Command::new(command)
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

    let mut line_index: usize = 0;
    let reader = BufReader::new(stdout);
    reader
        .lines()
        // TODO Is it the best way to achieve that?
        .filter_map(|line| line.ok())
        .for_each({
            move |line| {
                #[cfg(debug_assertions)]
                {
                    println!("[cli::run()] {}", line);
                }

                match &callback_option {
                    Some(callback) => {
                        callback(line_index, line);

                        line_index += 1;
                    }
                    None => (),
                }
            }
        });
}

#[cfg(not(tarpaulin_include))]
pub fn run_in_thread<C>(command: String, args: Vec<String>, callback_option: Option<C>) -> ()
where
    C: Fn(usize, String) -> (),
    C: Send + 'static,
{
    let join_handle = thread::spawn(move || {
        run(command, args, callback_option);
    });

    join_handle.join().unwrap();
}
