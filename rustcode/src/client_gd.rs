use std::time::Duration;
use std::time::Instant;

use bincode::Options;
use godot::classes::INode2D;
use godot::classes::Node2D;
use godot::prelude::*;
use godot_tokio::AsyncRuntime;
use ringbuffer::RingBuffer;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;

use crate::player;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Client {
    base: Base<Node2D>,
    tick: usize,
    player: Option<Gd<player::Player>>,
    client: Arc<Mutex<client::Client>>,
}

#[godot_api]
impl INode2D for Client {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Client init");

        Self {
            base,
            tick: 0,
            player: None,
            client: Arc::new(Mutex::new(client::Client::new())),
        }
    }

    fn ready(&mut self) {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();

        // Set priority, so the player Node's process function run earlier
        self.base_mut().set_process_priority(10);
        self.player = Some(self.base().get_node_as::<player::Player>("Player"));
    }

    fn process(&mut self, delta: f64) {
        let mut player = self
            .player
            .as_mut()
            .expect("Player node not found!")
            .bind_mut();

        // For testing purpose!
        // assert!(self.tick > player.tick);

        // Send new actions to server
        if self.tick % 6 == 0 {
            // Collect unprocessed player actions
            let mut actions = vec![];
            for action in player.actions.iter_mut() {
                match action {
                    common::Action::Movement(movement) => {
                        if movement.state == common::ActionState::New {
                            movement.state = common::ActionState::SentByClient;
                            actions.push(action.clone()) // TODO
                        }
                    }
                }
            }

            // Empty action vector is also sent to server otherwise it disconnects in some seconds
            // if !actions.is_empty() {
            let client = self.client.clone();
            AsyncRuntime::spawn(async move {
                let start = Instant::now();
                match client.try_lock() {
                    Ok(mut client) => {
                        // Update client
                        client
                            .update(Duration::from_secs_f64(6.0 * delta))
                            // In AsyncRuntime scopes, godo_print/error/etc. does not work!
                            .map_err(|e| tracing::error!("Error during client update: {:?}", e))
                            .ok();

                        // Send data to server
                        let frame = common::Frame::new(actions);
                        let payload = bincode::options().serialize(&frame).unwrap();
                        client.send(payload);

                        // Get data from server
                        let new_frame = client.get_messages();

                        // tracing::debug!("Handling networking took: {:?}", start.elapsed(),);
                    }
                    Err(_) => tracing::warn!("Networking seem to be slow, change something?!"),
                }
            });
            // }
        }

        self.tick += 1;
    }
}

#[godot_api]
impl Client {}
