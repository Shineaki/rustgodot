use godot::{builtin::Vector2, classes::CharacterBody2D, obj::Gd};
use ringbuffer::AllocRingBuffer;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

pub const BUFFER_CAPACITY: usize = 32;
pub const PORT: usize = 7614;
pub const PROTOCOL_ID: u64 = 27;
pub const PLAYER_SCENE_PATH: &str = "res://Scenes/Player.tscn";

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
    pub pos: (f32, f32),
    pub timestamp: Duration,
    pub state: ActionState,
}

impl Movement {
    pub fn new(tick: usize, delta: f64, input: (i8, i8), pos: Vector2) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        Self {
            tick,
            delta,
            input,
            pos: (pos.x, pos.y),
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
    InValidatedByServer,
    ProcessedByClient,
}

pub struct ServerSidePlayerData {
    pub speed: f32,
    pub player: Gd<CharacterBody2D>,
    pub messages: AllocRingBuffer<Action>,
}

impl ServerSidePlayerData {
    pub fn new(character: Gd<CharacterBody2D>) -> Self {
        Self {
            speed: 100.0,
            player: character,
            messages: AllocRingBuffer::new(BUFFER_CAPACITY),
        }
    }
}

pub fn player_movement(player: &mut Gd<CharacterBody2D>, input: (i8, i8), speed: f32, delta: f64) {
    player.move_and_collide(
        Vector2::new(input.0 as f32, input.1 as f32).normalized() * speed * delta as f32,
    );
}
