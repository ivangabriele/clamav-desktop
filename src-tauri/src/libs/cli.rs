use std::process::Stdio;
use tokio::process::{Child, Command};

// #[derive(Clone, serde::Serialize)]
// struct LogPayload {
//     logs: Vec<String>,
// }

#[cfg(not(tarpaulin_include))]
#[allow(dead_code)]
pub async fn run(binary_path: String, args: Vec<String>) -> Child {
    Command::new(binary_path)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
}
