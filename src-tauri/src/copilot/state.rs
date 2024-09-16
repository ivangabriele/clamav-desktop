use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

use crate::{debug, globals};

use super::*;

#[derive(Default)]
pub struct CopilotSharedState(pub CopilotState);

#[derive(Default)]
pub struct CopilotState {
    // pub private: CopilotPrivateState,
    pub public: Arc<Mutex<CopilotPublicState>>,
}

#[derive(Default)]
pub struct CopilotPrivateState {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CopilotPublicState {
    pub current_checklist_error: Option<String>,
    pub current_checklist_item: Option<constants::ChecklistItem>,
    /// A floating number between 0 and 1 representing the current progress of the checklist.
    pub current_checklist_progress: f32,
    pub is_fixing_current_checklist_item: bool,
    pub module_status: globals::ModuleStatus,
}

pub async fn broadcast_state(app_handle: &AppHandle) {
    debug!("broadcast_state()", "Function call.");

    let public_state_mutex_guard = app_handle.state::<CopilotSharedState>().inner().0.public.lock().await;

    app_handle
        .emit("copilot:state", public_state_mutex_guard.clone())
        .unwrap();
}

pub async fn set_public_state(app_handle: &AppHandle, next_public_state: CopilotPublicState, with_broadcast: bool) {
    debug!("set_public_state_checklist_error()", "Function call.");

    let mut public_state_mutex_guard = app_handle.state::<CopilotSharedState>().inner().0.public.lock().await;

    *public_state_mutex_guard = next_public_state;
    drop(public_state_mutex_guard);

    if with_broadcast {
        broadcast_state(app_handle).await;
    }
}

pub async fn set_public_state_current_checklist_error(
    app_handle: &AppHandle,
    next_checklist_error: Option<String>,
    with_broadcast: bool,
) {
    debug!("set_public_state_checklist_error()", "Function call.");

    let mut public_state_mutex_guard = app_handle.state::<CopilotSharedState>().inner().0.public.lock().await;

    public_state_mutex_guard.current_checklist_error = next_checklist_error;
    drop(public_state_mutex_guard);

    if with_broadcast {
        broadcast_state(app_handle).await;
    }
}

pub async fn set_public_state_current_checklist_item(
    app_handle: &AppHandle,
    next_checklist_item: Option<&constants::ChecklistItem>,
    with_broadcast: bool,
) {
    debug!("set_public_state_current_checklist_item()", "Function call.");

    let mut public_state_mutex_guard = app_handle.state::<CopilotSharedState>().inner().0.public.lock().await;

    public_state_mutex_guard.current_checklist_item = next_checklist_item.cloned();
    drop(public_state_mutex_guard);

    if with_broadcast {
        broadcast_state(app_handle).await;
    }
}

pub async fn set_public_state_current_checklist_progress(
    app_handle: &AppHandle,
    next_checklist_progress: f32,
    with_broadcast: bool,
) {
    debug!("set_public_state_current_checklist_progress()", "Function call.");

    let mut public_state_mutex_guard = app_handle.state::<CopilotSharedState>().inner().0.public.lock().await;

    public_state_mutex_guard.current_checklist_progress = next_checklist_progress;
    drop(public_state_mutex_guard);

    if with_broadcast {
        broadcast_state(app_handle).await;
    }
}

pub async fn set_public_state_is_fixing_current_checklist_item(
    app_handle: &AppHandle,
    next_is_fixing_current_checklist_item: bool,
    with_broadcast: bool,
) {
    debug!("set_public_state_is_fixing_current_checklist_item()", "Function call.");

    let mut public_state_mutex_guard = app_handle.state::<CopilotSharedState>().inner().0.public.lock().await;

    public_state_mutex_guard.is_fixing_current_checklist_item = next_is_fixing_current_checklist_item;
    drop(public_state_mutex_guard);

    if with_broadcast {
        broadcast_state(app_handle).await;
    }
}

pub async fn set_public_state_module_status(
    app_handle: &AppHandle,
    next_module_status: globals::ModuleStatus,
    with_broadcast: bool,
) {
    debug!("set_public_state_module_status()", "Function call.");

    let mut public_state_mutex_guard = app_handle.state::<CopilotSharedState>().inner().0.public.lock().await;

    public_state_mutex_guard.module_status = next_module_status;
    drop(public_state_mutex_guard);

    if with_broadcast {
        broadcast_state(app_handle).await;
    }
}
