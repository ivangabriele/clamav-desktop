use lazy_static::lazy_static;
use std::{path::PathBuf, sync::Mutex};

lazy_static! {
    pub static ref DATA_DIRECTORY_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    pub static ref LOG_DIRECTORY_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
}
