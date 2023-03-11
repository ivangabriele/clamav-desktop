use std::sync::Mutex;

use crate::scanner;

pub struct CoreState {
    pub scanner: scanner::ScannerState,
}

pub struct CoreStateMutex(pub Mutex<CoreState>);
