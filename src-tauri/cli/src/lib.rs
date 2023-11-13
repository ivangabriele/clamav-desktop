use colored::Colorize;
use std::{
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

use common;
use error::CliError;

mod error;

pub fn exec<C, A>(command: C, args: Vec<A>) -> Result<String, CliError>
where
    C: AsRef<str>,
    A: AsRef<str>,
{
    let command_as_string = command.as_ref().to_string();
    let args_as_strings: Vec<String> = args.iter().map(|arg| arg.as_ref().to_string()).collect();

    match Command::new(command_as_string.to_owned())
        .args(args_as_strings.to_owned())
        .output()
    {
        Ok(output) => {
            let stdout_as_string = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr_as_string = String::from_utf8_lossy(&output.stderr).trim().to_string();

            if stdout_as_string.is_empty() {
                return Err(CliError::new(
                    format!(
                        "`Command.output()` failed with command {} and args {:?}.",
                        command_as_string.to_owned(),
                        args_as_strings.to_owned()
                    ),
                    Box::new(common::CommonError::new(stderr_as_string)),
                ));
            }

            return Ok(stdout_as_string);
        }
        Err(error) => Err(CliError::new(
            format!(
                "`Command.output()` failed with command {} and args {:?}.",
                command_as_string.to_owned(),
                args_as_strings.to_owned()
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

pub fn run<C, A, CB1, CB2>(
    command: C,
    args: Vec<A>,
    stdout_callback: CB1,
    stderr_callback: CB2,
) -> ()
where
    C: AsRef<str>,
    A: AsRef<str>,
    CB1: Fn(usize, String) -> (),
    CB1: Send + 'static,
    CB2: Fn(usize, String) -> (),
    CB2: Send + 'static,
{
    let command_as_string = command.as_ref();
    let args_as_strings: Vec<&str> = args.iter().map(AsRef::as_ref).collect();

    let child = match Command::new(command_as_string)
        .args(args_as_strings)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            println!("{:?}", error);

            panic!("Bye");
        }
    };

    let mut line_index: usize = 0;

    let stderr = match child
        .stderr
        .ok_or_else(|| println!("Could not capture standard error."))
    {
        Ok(stderr) => stderr,
        Err(error) => {
            println!("{:?}", error);

            panic!("Bye");
        }
    };
    let stderr_reader = BufReader::new(stderr);
    stderr_reader
        .lines()
        // TODO Is it the best way to achieve that?
        .filter_map(|line| line.ok())
        .for_each({
            move |line| {
                #[cfg(debug_assertions)]
                {
                    println!(
                        "{} {} {}",
                        "[DEBUG]".cyan(),
                        "[cli::run()]".cyan(),
                        line.underline()
                    );
                }

                stdout_callback(line_index, line);

                line_index += 1;
            }
        });

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
    let stdout_reader = BufReader::new(stdout);
    stdout_reader
        .lines()
        // TODO Is it the best way to achieve that?
        .filter_map(|line| line.ok())
        .for_each({
            move |line| {
                #[cfg(debug_assertions)]
                {
                    println!("{} {} {}", "[DEBUG]".cyan(), "[cli::run()]".cyan(), line);
                }

                stderr_callback(line_index, line);

                line_index += 1;
            }
        });
}

#[cfg(not(tarpaulin_include))]
pub fn run_in_thread<C, A, CB1, CB2>(
    command: C,
    args: Vec<A>,
    stdout_callback: CB1,
    stderr_callback: CB2,
) -> ()
where
    C: AsRef<str> + 'static + std::marker::Send,
    A: AsRef<str> + 'static + std::marker::Send,
    CB1: Fn(usize, String) -> (),
    CB1: Send + 'static,
    CB2: Fn(usize, String) -> (),
    CB2: Send + 'static,
{
    let join_handle = thread::spawn(move || {
        run(command, args, stdout_callback, stderr_callback);
    });

    join_handle.join().unwrap();
}
