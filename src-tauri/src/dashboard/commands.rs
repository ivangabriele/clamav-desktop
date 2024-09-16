use futures_util::{SinkExt, StreamExt};
use std::process::Stdio;
use tauri::{AppHandle, State};
use tokio::process::Command;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{http::Uri, Message},
};

use crate::debug;

use super::*;

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_dashboard_state(
    app_handle: AppHandle,
    shared_state: State<'_, state::DashboardSharedState>,
) -> Result<(), ()> {
    use tauri::Emitter;

    debug!("get_dashboard_state()", "Command call.");

    let url = Uri::from_static("ws://127.0.0.1:7878");
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    let client_message = types::ClientMessage {
        id: cuid::cuid1().expect("Failed to generate CUID."),
        action: "Ping".to_string(),
        data: serde_json::json!({}),
    };
    let client_message_as_string = serde_json::to_string(&client_message).unwrap();
    write
        .send(Message::Text(client_message_as_string))
        .await
        .expect("Failed to send status command");

    match read.next().await {
        Some(Ok(daemon_message)) => {
            let daemon_message_as_text = daemon_message
                .to_text()
                .expect("Failed to convert `daemon_message` to text.")
                .to_string();
            let daemon_message: types::DaemonMessage = serde_json::from_str(&daemon_message_as_text)
                .expect("Failed to convert `daemon_message_as_text` to `DaemonMessage`.");
            println!("Received a message from daemon: `{:?}`.", daemon_message);

            let mut public_state_mutex_guard = shared_state.0.public.lock().await;

            let (status, logs) = utils::get_service_status();
            let next_public_state = state::DashboardPublicState {
                is_ready: true,
                logs,
                status,
            };
            *public_state_mutex_guard = next_public_state.clone();
            app_handle.emit("dashboard:state", &next_public_state).unwrap();

            return Ok(());
        }
        Some(Err(e)) => {
            println!("Error: {:?}", e);

            return Err(());
        }
        None => {
            println!("No message received.");

            return Err(());
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn start_daemon() -> Result<(), ()> {
    debug!("start_daemon()", "Command call.");

    Command::new("systemctl")
        .args(["--no-pager", "start", "clamav-daemon"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl --no-pager stop clamav-daemon`");

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn stop_daemon() -> Result<(), ()> {
    debug!("stop_daemon()", "Command call.");

    Command::new("systemctl")
        .args(["--no-pager", "stop", "clamav-daemon"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run `systemctl --no-pager stop clamav-daemon`");

    Ok(())
}
