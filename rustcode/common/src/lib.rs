use godot::{
    builtin::Vector2,
    classes::CharacterBody2D,
    obj::{Gd, NewAlloc},
};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

pub const BUFFER_CAPACITY: usize = 32;
pub const PORT: usize = 7614;
pub const PROTOCOL_ID: u64 = 27;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub timestamp: Duration,
    pub messages: Vec<Action>,
}

impl Frame {
    pub fn new(messages: Vec<Action>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        Self {
            timestamp,
            messages,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Movement(Movement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    pub tick: usize,
    pub delta: f64,
    pub input: (i8, i8),
    pub timestamp: Duration,
    pub state: ActionState,
}

impl Movement {
    pub fn new(tick: usize, delta: f64, input: (i8, i8)) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        Self {
            tick,
            delta,
            input,
            timestamp,
            state: ActionState::New,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ActionState {
    New,
    SentByClient,
    ValidatedByServer,
    ProcessedByClient,
}

pub struct ServerSidePlayerData {
    pub speed: f32,
    pub player: Gd<CharacterBody2D>,
    pub messages: AllocRingBuffer<Action>,
}

impl ServerSidePlayerData {
    pub fn new() -> Self {
        Self {
            speed: 100.0,
            player: CharacterBody2D::new_alloc(),
            messages: AllocRingBuffer::new(BUFFER_CAPACITY),
        }
    }
}
