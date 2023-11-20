use filer;
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicBool, Arc};
use tokio::{process::Child, sync::Mutex};

#[derive(Default)]
pub struct SharedScannerState(pub ScannerState);

#[derive(Default)]
pub struct ScannerState {
    pub private: ScannerPrivateState,
    pub public: Arc<Mutex<ScannerPublicState>>,
}

#[derive(Default)]
pub struct ScannerPrivateState {
    pub child: Arc<Mutex<Option<Child>>>,
    pub child_id: Arc<Mutex<Option<u32>>>,
    pub should_stop: Arc<AtomicBool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ScannerPublicState {
    pub file_explorer_tree: filer::file_explorer::FileExplorerTree,
    pub is_ready: bool,
    pub is_running: bool,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ScannerStatus {
    pub current_file_path: String,
    pub progress: f64,
    pub step: ScannerStatusStep,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ScannerStatusStep {
    /// Counting files to scan.
    Counting,
    /// Default step (= waiting for a new job).
    #[default]
    Idle,
    /// Listing files to scan.
    Listing,
    /// Scanning files.
    Running,
    /// Starting (= has called `clamscan` CLI command).
    Starting,
    /// Stopping (= has called `stop_scanner()` Tauri command).
    Stopping,
}
