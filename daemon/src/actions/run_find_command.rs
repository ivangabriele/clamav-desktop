use serde_json::Value;
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub async fn run_find_command(data: Value) {
    let path = data["path"].as_str().expect("`data.path` not provided.");
    let name = data["name"].as_str().expect("`data.name` not provided.");

    println!(
        "Running `find` command with path: `{}` and name: `{}`.",
        path, name
    );

    let mut child = Command::new("find")
        .args([path, "-name", name])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn `find` command.");

    let stdout_buf_reader = BufReader::new(child.stdout.take().expect("Failed to take stdout."));
    let mut stdout_lines = stdout_buf_reader.lines();

    let stderr_buf_reader = BufReader::new(child.stderr.take().expect("Failed to take stderr."));
    let mut stderr_lines = stderr_buf_reader.lines();

    loop {
        tokio::select! {
            stdout_line = stdout_lines.next_line() => {
                if let Ok(Some(line)) = stdout_line {
                    println!("stdout: {}", line);
                }
            },
            stderr_line = stderr_lines.next_line() => {
                if let Ok(Some(line)) = stderr_line {
                    println!("stderr: {}", line);
                }
            },
            else => break,
        }
    }
}
