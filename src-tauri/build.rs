use std::fs;

fn main() {
    match fs::create_dir_all("../build") {
        Ok(..) => (),
        Err(..) => panic!("Unable to create '../build' directory."),
    };

    tauri_build::build()
}
