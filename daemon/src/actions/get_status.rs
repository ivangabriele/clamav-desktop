use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{state, types};

pub async fn get_status(
    client_message: types::ClientMessage,
    state: Arc<Mutex<state::DaemonState>>,
) -> types::DaemonMessage {
    println!("[ACTION] get_status()");

    let state_mutex_guard = state.lock().await;
    let status = state_mutex_guard.get_status();
    drop(state_mutex_guard);

    let server_response = types::DaemonMessage {
        id: cuid::cuid1().expect("Failed to generate CUID."),
        client_message_id: client_message.id,
        action: types::DaemonAction::ReturnStatus,
        data: Some(serde_json::json!({ "status": status })),
    };

    server_response
}
