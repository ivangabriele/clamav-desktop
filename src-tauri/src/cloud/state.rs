use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct CloudSharedState(pub CloudState);

#[derive(Default)]
pub struct CloudState {
    pub private: CloudPrivateState,
    pub public: Arc<Mutex<CloudPublicState>>,
}

#[derive(Default)]
pub struct CloudPrivateState {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CloudPublicState {
    pub status: CloudDaemonStatus,
    pub is_ready: bool,
    pub is_running: bool,
    pub logs: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum CloudDaemonStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}
