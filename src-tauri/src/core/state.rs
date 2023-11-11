use std::{
    process::Child,
    sync::{Arc, Mutex},
};

use crate::{cloud, daemon, scanner};

#[derive(Default)]
pub struct CoreState {
    pub cloud: cloud::CloudState,
    pub daemon: daemon::DaemonState,
    pub scanner: scanner::ScannerState,
    pub scanner_thread: Option<Child>,
}

#[derive(Default)]
pub struct SharedCoreState(pub Arc<Mutex<CoreState>>);
