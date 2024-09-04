use crate::types;

pub async fn ping(client_message: types::ClientMessage) -> types::DaemonMessage {
    println!("[ACTION] ping()");

    let server_response = types::DaemonMessage {
        id: cuid::cuid1().expect("Failed to generate CUID."),
        client_message_id: client_message.id,
        action: types::DaemonAction::Pong,
        data: None,
    };

    server_response
}
