use chrono::Utc;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::globals::LOG_DIRECTORY_PATH;

/// Write a log message to a specified log file.
pub async fn write_log_message(log_file_name: &str, scope: &str, message: &str) {
    let log_directory_path = LOG_DIRECTORY_PATH.lock().await;
    let log_directory_path_clone = log_directory_path.clone();
    if !log_directory_path_clone.exists() {
        fs::create_dir_all(&log_directory_path_clone).expect("Failed to create log directory.");
    }

    let log_file_path_as_path_buf = log_directory_path.join(log_file_name);
    let log_file_path_as_str = log_file_path_as_path_buf
        .to_str()
        .expect("Failed to convert log file path to string.");

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path_as_str)
        .expect(format!("Failed to open `{}` file.", log_file_path_as_str).as_str());
    writeln!(
        log_file,
        "{},{},{}",
        Utc::now().to_rfc3339(),
        scope,
        message
    )
    .expect("Failed to write to log file.");
}

/// Log a debug message.
///
/// This macro logs a message both to the console in blue text and to a CSV file with a timestamp.
/// The message format and arguments are similar to `println!`.
///
/// # Examples
///
/// ```
/// # use logger::debug;
///
/// debug!("my_function()", "Value: {}", 42);
/// ```
#[macro_export]
macro_rules! debug {
    ($function_name:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        let scope = format!("{}::{}", module_path!(), $function_name);

        #[cfg(debug_assertions)]
        {
            println!("\x1b[0;34m[DEBUG] [{}] {}\x1b[0m", scope, message);
        }

        tokio::spawn(async move {
            crate::libs::logger::write_log_message("debug.csv", &scope, &message).await;
        });
    }};
}

/// Log an error message.
///
/// This macro logs a message both to the console in red text and to a CSV file with a timestamp.
/// The message format and arguments are similar to `println!`.
///
/// # Examples
///
/// ```
/// # use logger::error;
///
/// error!("my_function()", "Value: {}", 42);
/// ```
#[macro_export]
macro_rules! error {
    ($function_name:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        let scope = format!("{}::{}", module_path!(), $function_name);

        println!("\x1b[0;0;31m[ERROR] [{}] {}\x1b[0m", scope, message);

        tokio::spawn(async move {
            crate::libs::logger::write_log_message("error.csv", &scope, &message).await;
        });
    }};
}
