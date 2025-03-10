use std::time::Duration;
use std::time::Instant;

use bincode::Options;
use godot::classes::{INode2D, Node2D};
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
    server_actions: Arc<Mutex<Vec<common::Action>>>,
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
            server_actions: Arc::new(Mutex::new(vec![])),
        }
    }

    fn ready(&mut self) {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();

        // Set priority, so the player Node's process function runs earlier
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
        // TODO: should be based on FPS
        if self.tick % 6 == 0 {
            // Collect unprocessed player actions
            let actions = Client::get_unprocessed_actions(&mut player);

            let server_actions = self.server_actions.clone();
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
                        if let Some(v_actions) = client.get_messages() {
                            server_actions.lock().unwrap().extend(v_actions.messages);
                        }

                        // tracing::debug!("Handling networking took: {:?}", start.elapsed(),);
                    }
                    Err(_) => tracing::warn!("Networking seem to be slow, change something?!"),
                }
            });
            // }

            let asd = self.server_actions.clone();
            {
                let v_actions = asd.lock().unwrap();
                if !v_actions.is_empty() {
                    if let common::Action::Movement(latest_movement) =
                        v_actions.iter().last().unwrap()
                    {
                        match latest_movement.state {
                            common::ActionState::ValidatedByServer => {}
                            common::ActionState::InValidatedByServer => {
                                player.base_mut().set_position(Vector2::new(
                                    latest_movement.pos.0,
                                    latest_movement.pos.1,
                                ));
                            }
                            _ => unreachable!(),
                        }

                        // let mut m = common::Movement::new(0, 0.0, (0, 0), Vector2::new(0.0, 0.0));
                        // for a in player.actions.iter() {
                        //     match a {
                        //         common::Action::Movement(movement) => {
                        //             if movement.tick == latest_movement.tick {
                        //                 m = movement.clone();
                        //                 tracing::debug!("FOUND!!!");
                        //                 break;
                        //             }
                        //         }
                        //     }
                        // }
                    }
                }
            }
        }

        self.tick += 1;
    }
}

#[godot_api]
impl Client {
    fn get_unprocessed_actions(player: &mut player::Player) -> Vec<common::Action> {
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

        actions
    }
}
