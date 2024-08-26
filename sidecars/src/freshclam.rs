mod common;

use common::execute_clamav_command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    execute_clamav_command("freshclam", args);
}
