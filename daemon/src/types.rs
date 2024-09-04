use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClientMessage {
    pub id: String,
    pub action: String,
    pub data: Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DaemonMessage {
    pub id: String,
    pub client_message_id: String,
    pub action: DaemonAction,
    pub data: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DaemonAction {
    Error,
    Pong,
    ReturnStatus,
}
