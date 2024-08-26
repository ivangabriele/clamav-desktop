use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct SharedSettingsState(pub SettingsState);

#[derive(Default)]
pub struct SettingsState {
    // pub private: SettingsPrivateState,
    pub public: Arc<Mutex<SettingsPublicState>>,
}

#[derive(Default)]
pub struct SettingsPrivateState {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SettingsPublicState {
    pub clamd_conf_file_path: Option<String>,
    pub clamd_conf_file_source: Option<String>,
    pub is_ready: bool,
    pub is_writing: bool,
}
