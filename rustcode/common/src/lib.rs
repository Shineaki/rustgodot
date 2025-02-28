use std::time::{Duration, SystemTime};

use godot::builtin::Vector2;
use renet::ClientId;
use serde::{Deserialize, Serialize};

pub const PORT: usize = 7614;
pub const PROTOCOL_ID: u64 = 27;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    // pub client_id: ClientId,
    // pub timestamp: Duration,
    pub messages: Vec<Message>,
}

impl Frame {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub client_id: ClientId,
    pub timestamp: Duration,
    pub action: ActionType,

    // #[serde(skip_serializing)]
    pub state: MessageState,
}

impl Message {
    pub fn new(client_id: ClientId, action: ActionType) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        Self {
            client_id,
            timestamp,
            action,
            state: MessageState::Created,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Movement(Movement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    pub input: (i8, i8),
    pub cur_ts: f64,
    pub prev_ts: f64,
    pub cur_pos: (f32, f32),
    pub prev_pos: (f32, f32),
}

impl Movement {
    pub fn new(
        input: (i8, i8),
        cur_ts: f64,
        prev_ts: f64,
        cur_pos: Vector2,
        prev_pos: Vector2,
    ) -> Self {
        Self {
            input,
            cur_ts,
            prev_ts,
            cur_pos: (cur_pos.x, cur_pos.y),
            prev_pos: (prev_pos.x, prev_pos.y),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MessageState {
    Created,
    Sent,
    ServerValidated,
}
