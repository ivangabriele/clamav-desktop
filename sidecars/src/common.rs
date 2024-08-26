use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn resolve_resource_path(binary_name: &str) -> PathBuf {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    if current_exe.parent().unwrap().ends_with("target") {
        // In development mode, use `../../../resources/clamav` as the base path
        let mut dev_path = PathBuf::from(current_exe.parent().unwrap());
        dev_path.pop(); // Go one level up
        dev_path.pop(); // Go another level up
        dev_path.pop(); // Go another level up
        dev_path.push(format!("resources/clamav/{}", binary_name));

        dev_path
    } else {
        // In production mode, use `./_up_/resources` as the base path
        let mut prod_path = PathBuf::from(current_exe.parent().unwrap());
        prod_path.push(format!("_up_/resources/clamav/{}", binary_name));

        prod_path
    }
}

pub fn execute_clamav_command(binary_name: &str, args: Vec<String>) {
    let resource_path = resolve_resource_path(binary_name);

    let status = Command::new(resource_path)
        .args(&args)
        .status()
        .expect(&format!("Failed to execute {}", binary_name));

    exit(status.code().unwrap_or(1));
}
