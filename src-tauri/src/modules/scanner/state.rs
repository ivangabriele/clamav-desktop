use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicBool, Arc};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::process::CommandChild;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct ScannerSharedState(pub ScannerState);

#[derive(Default)]
pub struct ScannerState {
    pub private: ScannerPrivateState,
    pub public: Arc<Mutex<ScannerPublicState>>,
}

#[derive(Default)]
pub struct ScannerPrivateState {
    pub child: Arc<Mutex<Option<CommandChild>>>,
    pub child_id: Arc<Mutex<Option<u32>>>,
    pub should_stop: Arc<AtomicBool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ScannerPublicState {
    pub current_path: Option<String>,
    pub progress: Option<f64>,
    pub step: ScannerStatusStep,
}
impl ScannerPublicState {
    pub fn patch(&mut self, patch: ScannerPublicStatePatch) {
        if let Some(current_path) = patch.current_path {
            self.current_path = current_path;
        }

        if let Some(progress) = patch.progress {
            self.progress = progress;
        }

        if let Some(step) = patch.step {
            self.step = step;
        }
    }
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

// -----------------------------------------------------------------------------
// Setters

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ScannerPublicStatePatch {
    pub current_path: Option<Option<String>>,
    pub progress: Option<Option<f64>>,
    pub step: Option<ScannerStatusStep>,
}

pub async fn broadcast_state(app_handle: &AppHandle) {
    let public_state_mutex_guard = app_handle.state::<ScannerSharedState>().inner().0.public.lock().await;

    app_handle
        .emit("scanner:state", public_state_mutex_guard.clone())
        .unwrap();
}

#[allow(dead_code)]
pub async fn patch_public_state(app_handle: &AppHandle, patch: ScannerPublicStatePatch) {
    let mut public_state_mutex_guard = app_handle.state::<ScannerSharedState>().inner().0.public.lock().await;

    let mut next_public_state = public_state_mutex_guard.clone();
    next_public_state.patch(patch);

    *public_state_mutex_guard = next_public_state;
    drop(public_state_mutex_guard);

    broadcast_state(app_handle).await;
}

pub async fn set_public_state(app_handle: &AppHandle, next_public_state: ScannerPublicState) {
    let mut public_state_mutex_guard = app_handle.state::<ScannerSharedState>().inner().0.public.lock().await;

    *public_state_mutex_guard = next_public_state;
    drop(public_state_mutex_guard);

    broadcast_state(app_handle).await;
}
