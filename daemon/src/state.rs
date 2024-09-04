use std::{collections::VecDeque, net::SocketAddr};

use crate::types;

#[derive(Clone, Debug)]
pub struct DaemonState {
    pub client: Option<SocketAddr>,
    pub currently_processed_message: Option<types::ClientMessage>,
    pub message_queue: VecDeque<types::ClientMessage>,
}
impl DaemonState {
    pub fn new() -> Self {
        Self {
            client: None,
            currently_processed_message: None,
            message_queue: VecDeque::new(),
        }
    }

    pub fn get_status(&self) -> String {
        let current = match &self.currently_processed_message {
            Some(cmd) => format!("Currently running: {:?}", cmd),
            None => "No command is currently running.".to_string(),
        };
        let queue = format!("Queued commands: {:?}", self.message_queue);
        format!("{}\n{}", current, queue)
    }
}
