use std::process::{Child, Command, Stdio};

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    logs: Vec<String>,
}

#[cfg(not(tarpaulin_include))]
pub fn run(binary_path: String, args: Vec<String>) -> Child {
    let child = Command::new(binary_path)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process.");

    child
}
