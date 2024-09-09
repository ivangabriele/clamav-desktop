use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

mod actions;
mod state;
mod types;

fn spawn_process_client_message(
    state: Arc<Mutex<state::DaemonState>>,
    client_message: types::ClientMessage,
    addr: SocketAddr,
    outgoing: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
) {
    tokio::spawn(async move {
        process_client_message(state, client_message, addr, outgoing).await;
    });
}

async fn process_client_message(
    state: Arc<Mutex<state::DaemonState>>,
    client_message: types::ClientMessage,
    addr: SocketAddr,
    outgoing: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
) {
    let mut state_mutex_guard = state.lock().await;
    state_mutex_guard.currently_processed_message = Some(client_message.clone());

    let state_traveller = state.clone();
    let server_response = match client_message.action.as_str() {
        "GetStatus" => actions::get_status(client_message, state_traveller).await,
        "Ping" => actions::ping(client_message).await,
        _ => {
            println!("[ERROR] Unknown action: `{}`", client_message.action);

            types::DaemonMessage {
                id: cuid::cuid1().expect("Failed to generate CUID."),
                client_message_id: client_message.id,
                action: types::DaemonAction::Error,
                data: Some(
                    serde_json::json!({ "message": format!("Unknown action `{}`.", client_message.action) }),
                ),
            }
        }
    };

    let mut outgoing_mutex_guard = outgoing.lock().await;
    outgoing_mutex_guard
        .send(Message::Text(
            serde_json::to_string(&server_response).unwrap(),
        ))
        .await
        .expect("Failed to send server response");
    drop(outgoing_mutex_guard);

    state_mutex_guard.currently_processed_message = None;
    if let Some(next_msg) = state_mutex_guard.message_queue.pop_front() {
        spawn_process_client_message(state.clone(), next_msg, addr, outgoing);
    }
}

async fn handle_client(
    state: Arc<Mutex<state::DaemonState>>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Incoming TCP connection from: {}", addr);

    let mut state_guard = state.lock().await;
    if state_guard.client.is_some() {
        println!(
            "Connection from {} rejected: another client is already connected.",
            addr
        );
        return;
    }
    state_guard.client = Some(addr);
    drop(state_guard);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("WebSocket handshake failed.");
    println!("WebSocket connection established: {}", addr);

    // Split the WebSocket stream into a read and write part
    let (outgoing, mut incoming) = ws_stream.split();
    let outgoing = Arc::new(Mutex::new(outgoing));

    while let Some(Ok(msg)) = incoming.next().await {
        let client_message_as_text = msg
            .to_text()
            .expect("Failed to convert `msg` to text.")
            .to_string();
        let client_message: types::ClientMessage = serde_json::from_str(&client_message_as_text)
            .expect("Failed to convert `client_message_as_text` to `ClientMessage`.");
        println!(
            "Received a message from `{}`: `{:?}`.",
            addr, client_message
        );

        let mut state_mutex_guard = state.lock().await;
        if state_mutex_guard.currently_processed_message.is_some() {
            state_mutex_guard.message_queue.push_back(client_message);
            drop(state_mutex_guard);
        } else {
            state_mutex_guard.currently_processed_message = Some(client_message.clone());
            drop(state_mutex_guard);

            tokio::spawn(process_client_message(
                state.clone(),
                client_message,
                addr,
                outgoing.clone(),
            ));
        }
    }

    println!("{} disconnected", &addr);
    let mut state_guard = state.lock().await;
    state_guard.client = None;
    drop(state_guard);
}

async fn start_server() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:7878".to_string());
    let state = Arc::new(Mutex::new(state::DaemonState::new()));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        handle_client(state.clone(), stream, addr).await;
    }
}

#[cfg(target_os = "windows")]
mod windows_service_handler {
    use std::ffi::OsString;
    use std::sync::mpsc::{self};
    use std::time::Duration;
    use tokio::runtime::Runtime;
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler, service_dispatcher, Result,
    };

    use crate::start_server;

    define_windows_service!(ffi_service_main, my_service_main);

    fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(e) = run_service() {
            eprintln!("Service failed: {}", e);
        }
    }

    fn run_service() -> windows_service::Result<()> {
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

        let event_handler = {
            let shutdown_tx = shutdown_tx.clone();
            move |control_event| -> service_control_handler::ServiceControlHandlerResult {
                match control_event {
                    ServiceControl::Stop => {
                        // Report that the service is stopping
                        shutdown_tx.send(()).unwrap();
                        service_control_handler::ServiceControlHandlerResult::NoError
                    }
                    ServiceControl::Interrogate => {
                        // Service is running, report no error
                        service_control_handler::ServiceControlHandlerResult::NoError
                    }
                    _ => service_control_handler::ServiceControlHandlerResult::NotImplemented,
                }
            }
        };

        // Register the control handler with Windows SCM
        let status_handle =
            service_control_handler::register("clamav-desktop-daemon", event_handler)?;

        // Report service is starting
        let start_pending_status = ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::StartPending,
            controls_accepted: ServiceControlAccept::STOP,
            process_id: None,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(30), // Hinting SCM that it will take time to start
        };
        status_handle.set_service_status(start_pending_status)?;

        // Start the Tokio runtime
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // Start the server asynchronously
            tokio::spawn(async {
                start_server().await;
            });
        });

        // Report service is now running
        let running_status = ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            process_id: None,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(0),
        };
        status_handle.set_service_status(running_status)?;

        // Block until we receive a shutdown signal
        shutdown_rx.recv().unwrap();

        // Report service is stopping
        let stop_pending_status = ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::StopPending,
            controls_accepted: ServiceControlAccept::empty(),
            process_id: None,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(30),
        };
        status_handle.set_service_status(stop_pending_status)?;

        // Properly clean up and stop the service
        let stop_status = ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            process_id: None,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(0),
        };
        status_handle.set_service_status(stop_status)?;

        Ok(())
    }

    pub fn run() -> Result<()> {
        service_dispatcher::start("clamav-desktop-daemon", ffi_service_main)?;

        Ok(())
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server().await;

    Ok(())
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    windows_service_handler::run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tokio::time::sleep;
    use tokio_tungstenite::{
        connect_async,
        tungstenite::{http::Uri, Message},
    };

    async fn is_port_in_use(port: u16) -> bool {
        match TcpListener::bind(("127.0.0.1", port)).await {
            Ok(_) => false,
            Err(_) => true,
        }
    }

    async fn wait_for_port(port: u16) {
        while !(is_port_in_use(port).await) {
            sleep(Duration::from_millis(100)).await;
        }
    }

    #[tokio::test]
    async fn test_ping_action() {
        tokio::spawn(start_server());
        wait_for_port(7878).await;

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

        if let Some(Ok(Message::Text(response_text))) = read.next().await {
            let daemon_response: types::DaemonMessage =
                serde_json::from_str(&response_text).unwrap();
            assert_eq!(daemon_response.client_message_id, client_message.id);
            assert_eq!(daemon_response.action, types::DaemonAction::Pong);
            assert!(daemon_response.data.is_none());
        } else {
            panic!("Did not receive the expected response.");
        }
    }
}
