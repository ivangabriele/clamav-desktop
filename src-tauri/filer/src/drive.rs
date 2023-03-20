use std::env;

use cli;
use common;

const EMPTY_STRING_VECTOR: Vec<String> = Vec::new();

pub fn list() -> Vec<String> {
    if cfg!(macos) {
        return vec!["/".to_string()];
    }

    if cfg!(unix) {
        return vec!["/".to_string()];
    }

    // TODO Use `winapi` for that (but it seems dead)?
    // https://docs.rs/winapi/latest/winapi/
    if cfg!(windows) {
        let args = common::utils::as_strings(vec!["logicaldisk", "get", "name"]);

        return match cli::exec("wmic".to_string(), args) {
            Ok(stdout) => {
                let mut drives: Vec<String> = stdout
                    .split("\n")
                    .map(|line_as_str| line_as_str.trim())
                    .filter(|line_as_str| !line_as_str.is_empty())
                    .map(String::from)
                    .collect();

                // We remove the command output header "Name"
                drives.remove(0);

                // TODO Split lines beforehand in internal `cli` lib via `BufReader.lines()`.
                drives
            }
            Err(..) => EMPTY_STRING_VECTOR,
        };
    }

    panic!("This OS is not supported: {}.", env::consts::OS)
}
