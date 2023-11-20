/// Logs a debug message.
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
    ($fn_name:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        use std::fs::{self, OpenOptions};
        use std::io::Write;

        use crate::globals::LOG_DIRECTORY_PATH;

        let log_directory_path = LOG_DIRECTORY_PATH.lock().expect("Could not lock log directory path.");
        let log_directory_path_clone = log_directory_path.clone();
        if !log_directory_path_clone.exists() {
            fs::create_dir_all(&log_directory_path_clone)
                .expect("Failed to create log directory.");
        }

        let log_file_path_as_path_buf = log_directory_path.join("debug.csv");
        let log_file_path_as_str = log_file_path_as_path_buf.to_str().expect("Failed to convert log file path to string.");
        let scope = format!("{}::{}", module_path!(), $fn_name);
        let message = format!($($arg)*);

        #[cfg(debug_assertions)]
        {
            println!("\x1b[0;34m[DEBUG] [{}] {}\x1b[0m", scope, message);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path_as_str)
            .expect("Failed to open debug.csv");
        writeln!(file, "{},{},{}", Utc::now().to_rfc3339(), scope, message)
            .expect("Failed to write to debug.csv");
    }};
}
