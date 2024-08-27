use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn resolve_binary_path(binary_name: &str) -> PathBuf {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let mut prod_path = PathBuf::from(current_exe.parent().unwrap());
    prod_path.push(format!("resources/clamav/{}", binary_name));

    prod_path
}

pub fn execute_clamav_command(binary_name: &str, args: Vec<String>) {
    let resource_path = resolve_binary_path(binary_name);

    let status = Command::new(resource_path)
        .args(&args)
        .status()
        .expect(&format!("Failed to execute {}", binary_name));

    exit(status.code().unwrap_or(1));
}
