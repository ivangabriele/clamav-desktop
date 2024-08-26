mod common;

use std::env;
use common::execute_clamav_command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    execute_clamav_command("clamscan", args);
}
