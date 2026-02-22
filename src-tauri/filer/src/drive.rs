use std::env;

use cli;
use common;

const EMPTY_STRING_VECTOR: Vec<String> = Vec::new();

pub fn list() -> Vec<String> {
    if cfg!(unix) {
        return vec!["/".to_string()];
    }

    if cfg!(windows) {
        let args = common::utils::as_strings(vec![
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_LogicalDisk | Select-Object -ExpandProperty Name",
        ]);

        return match cli::exec("powershell".to_string(), args) {
            Ok(stdout) => stdout
                .split("\n")
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(String::from)
                .collect(),
            Err(..) => EMPTY_STRING_VECTOR,
        };
    }

    panic!("This OS is not supported: {}.", env::consts::OS)
}
