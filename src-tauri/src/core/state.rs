use std::sync::Mutex;

use crate::{daemon, scanner};

pub struct CoreState {
    pub daemon: daemon::DaemonState,
    pub scanner: scanner::ScannerState,
}

pub struct CoreStateMutex(pub Mutex<CoreState>);
