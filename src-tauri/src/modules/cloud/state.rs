use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

use crate::globals;

#[derive(Default)]
pub struct CloudSharedState(pub CloudState);

#[derive(Default)]
pub struct CloudState {
    #[allow(dead_code)]
    pub private: CloudPrivateState,
    pub public: Arc<Mutex<CloudPublicState>>,
}

#[derive(Default)]
pub struct CloudPrivateState {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudPublicState {
    pub status: globals::ModuleStatus,
    pub is_up_to_date: Option<bool>,
}
impl CloudPublicState {
    pub fn patch(&mut self, patch: CloudPublicStatePatch) {
        if let Some(status) = patch.status {
            self.status = status;
        }

        if let Some(is_up_to_date) = patch.is_up_to_date {
            self.is_up_to_date = is_up_to_date;
        }
    }
}

// -----------------------------------------------------------------------------
// Setters

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudPublicStatePatch {
    pub status: Option<globals::ModuleStatus>,
    pub is_up_to_date: Option<Option<bool>>,
}

pub async fn broadcast_state(app_handle: &AppHandle) {
    let public_state_mutex_guard = app_handle.state::<CloudSharedState>().inner().0.public.lock().await;

    app_handle
        .emit("cloud:state", public_state_mutex_guard.clone())
        .unwrap();
}

pub async fn patch_public_state(app_handle: &AppHandle, patch: CloudPublicStatePatch) {
    let mut public_state_mutex_guard = app_handle.state::<CloudSharedState>().inner().0.public.lock().await;

    let mut next_public_state = public_state_mutex_guard.clone();
    next_public_state.patch(patch);

    *public_state_mutex_guard = next_public_state;
    drop(public_state_mutex_guard);

    broadcast_state(app_handle).await;
}

pub async fn set_public_state(app_handle: &AppHandle, next_public_state: CloudPublicState) {
    let mut public_state_mutex_guard = app_handle.state::<CloudSharedState>().inner().0.public.lock().await;

    *public_state_mutex_guard = next_public_state;
    drop(public_state_mutex_guard);

    broadcast_state(app_handle).await;
}
