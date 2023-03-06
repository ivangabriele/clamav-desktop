use std::{
    io::{BufRead, BufReader, Error, ErrorKind},
    process::{Command, Stdio},
    thread,
};

use crate::freshclam;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub fn start_clamscan(path: &str) -> Result<(), String> {
    println!("Start clamscan in {}.", path);

    // thread::scope(|scope| {
    //     scope::spawn(move || {
    //         let _result = clamscan::run(path);
    //     });
    // });
}

#[tauri::command]
pub fn start_freshclam() {
    println!("Start freshclam.");

    thread::spawn(|| {
        let _result = freshclam::run();
    });
}
