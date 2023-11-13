use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use tokio;
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() {
    let iterations = 10;
    let mut total_duration_a = std::time::Duration::new(0, 0);
    let mut total_duration_b = std::time::Duration::new(0, 0);
    let mut total_duration_c = std::time::Duration::new(0, 0);
    let mut total_duration_d = std::time::Duration::new(0, 0);
    let mut total_duration_e = std::time::Duration::new(0, 0);

    for index in 0..iterations {
        println!("{}", "=".to_string().repeat(80));
        println!("Iteration: {}", index + 1);
        println!("{}", "=".to_string().repeat(80));

        println!("{}", "-".to_string().repeat(80));
        println!("CASE A");
        println!("{}", "-".to_string().repeat(80));
        let start_a = Instant::now();
        run_a();
        total_duration_a += start_a.elapsed();

        println!("{}", "-".to_string().repeat(80));
        println!("CASE B");
        println!("{}", "-".to_string().repeat(80));
        let start_b = Instant::now();
        run_b();
        total_duration_b += start_b.elapsed();

        println!("{}", "-".to_string().repeat(80));
        println!("CASE C");
        println!("{}", "-".to_string().repeat(80));
        let start_c = Instant::now();
        run_c();
        total_duration_c += start_c.elapsed();

        println!("{}", "-".to_string().repeat(80));
        println!("CASE D");
        println!("{}", "-".to_string().repeat(80));
        let start_d = Instant::now();
        run_d().await;
        total_duration_d += start_d.elapsed();

        println!("{}", "-".to_string().repeat(80));
        println!("CASE E");
        println!("{}", "-".to_string().repeat(80));
        let start_e = Instant::now();
        run_e().await;
        total_duration_e += start_e.elapsed();
    }

    let average_duration_a = total_duration_a / iterations;
    let average_duration_b = total_duration_b / iterations;
    let average_duration_c = total_duration_c / iterations;
    let average_duration_d = total_duration_d / iterations;
    let average_duration_e = total_duration_e / iterations;

    println!("Average time for run_a: {:?}", average_duration_a);
    println!("Average time for run_b: {:?}", average_duration_b);
    println!("Average time for run_c: {:?}", average_duration_c);
    println!("Average time for run_d: {:?}", average_duration_d);
    println!("Average time for run_e: {:?}", average_duration_e);
}

fn run_a() {
    let child = Command::new("find")
        .args(["/home/ivan", "-name", "a"])
        .stdout(Stdio::piped()) // Pipe the standard output of the child process
        .spawn()
        .expect("Failed to start command");

    child.stdout.expect("Failed to open stdout");
}

fn run_b() {
    let child = Command::new("find")
        .args(["/home/ivan", "-name", "a"])
        .stdout(Stdio::piped()) // Pipe the standard output of the child process
        .spawn()
        .expect("Failed to start command");

    let stdout = child.stdout.expect("Failed to open stdout");

    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line), // Print each line as it comes in
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn run_c() {
    let (tx, rx) = mpsc::channel();

    // Spawn the command in a new thread
    thread::spawn(move || {
        let child = Command::new("find")
            .args(["/home/ivan", "-name", "a"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        let stdout = BufReader::new(child.stdout.expect("Failed to open stdout"));

        for line in stdout.lines() {
            if let Ok(line) = line {
                tx.send(line).expect("Failed to send line");
            }
        }
    });

    // Periodically process the last line
    let mut last_line = String::new();
    loop {
        while let Ok(line) = rx.try_recv() {
            last_line = line;
        }

        if !last_line.is_empty() {
            println!("Last line: {}", last_line);
            last_line.clear();
        }

        match rx.try_recv() {
            Err(TryRecvError::Disconnected) => break, // Exit the loop if the channel is closed
            _ => thread::sleep(Duration::from_secs(1)), // Otherwise, sleep and continue
        }
    }
}

async fn run_d() {
    let mut child = tokio::process::Command::new("find")
        .args(["/home/ivan", "-name", "a"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let stdout = tokio::io::BufReader::new(child.stdout.take().expect("Failed to open stdout"));
    let mut lines = stdout.lines();

    while let Some(line) = lines.next_line().await.expect("Failed to read line") {
        println!("{}", line);
    }
}

async fn run_e() {
    let (tx, rx) = mpsc::channel();
    let stdout = std::io::stdout();
    let mut stdout_lock = BufWriter::new(stdout.lock());

    // Spawn the command in a new thread
    thread::spawn(move || {
        let child = Command::new("find")
            .args(["/home/ivan", "-name", "a"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        let stdout = BufReader::new(child.stdout.expect("Failed to open stdout"));
        let mut buffer = Vec::new();

        for line in stdout.lines() {
            if let Ok(line) = line {
                buffer.push(line);
                // Send in chunks
                if buffer.len() >= 10 {
                    tx.send(buffer.clone()).expect("Failed to send buffer");
                    buffer.clear();
                }
            }
        }
        // Send any remaining lines
        if !buffer.is_empty() {
            tx.send(buffer).expect("Failed to send buffer");
        }
    });

    // Receive and print in chunks
    while let Ok(lines) = rx.recv() {
        for line in lines {
            writeln!(stdout_lock, "{}", line).expect("Failed to write to stdout");
        }
    }
}
