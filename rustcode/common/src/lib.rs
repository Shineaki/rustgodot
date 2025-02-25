use std::{collections::HashMap, time::{Duration, SystemTime}};

use renet::ClientId;
use renet_netcode::NETCODE_USER_DATA_BYTES;
use serde::{Deserialize, Serialize};

pub const PORT: usize = 7614;
pub const PROTOCOL_ID: u64 = 27;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    client_id: ClientId,
    timestamp: Duration,
    message_type: MessageType,
    payload: InputChanged
}

impl Message {
    pub fn new(client_id: ClientId, message_type: MessageType, payload: InputChanged) -> Self {
        let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

        Self { client_id, timestamp, message_type, payload }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    InputChanged
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputChanged {
    input: (i8, i8)
}

impl InputChanged {
    pub fn new(input: (i8, i8)) -> Self {
        Self { input }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessages {
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessages {
    ClientConnected {
        client_id: ClientId,
        username: String,
    },
    ClientDisconnected {
        client_id: ClientId,
    },
    ClientMessage(Message),
    InitClient {
        usernames: HashMap<ClientId, String>,
    },
}

pub struct Username(pub String);

impl Username {
    pub fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
            panic!("Username is too big");
        }
        user_data[0] = self.0.len() as u8;
        user_data[1..self.0.len() + 1].copy_from_slice(self.0.as_bytes());

        user_data
    }

    pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let mut len = user_data[0] as usize;
        len = len.min(NETCODE_USER_DATA_BYTES - 1);
        let data = user_data[1..len + 1].to_vec();
        let username = String::from_utf8(data).unwrap_or("unknown".to_string());
        Self(username)
    }
}
