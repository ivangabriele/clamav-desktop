use serde::{Deserialize, Serialize};
use std::{str, sync::Arc};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct DashboardSharedState(pub DashboardState);

#[derive(Default)]
pub struct DashboardState {
    // pub private: DashboardPrivateState,
    pub public: Arc<Mutex<DashboardPublicState>>,
}

#[derive(Default)]
pub struct DashboardPrivateState {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DashboardPublicState {
    pub is_ready: bool,
    pub logs: Vec<String>,
    pub status: DashboardStatus,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum DashboardStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}
