use godot::classes::{CharacterBody2D, INode2D, Node2D};
use godot::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
// use itertools::Itertools;
use renet::ServerEvent;
use ringbuffer::RingBuffer;
use tracing_subscriber::EnvFilter;

// TODO: clock synchronization!

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Server {
    server: Option<server::Server>,
    player_data: HashMap<u64, common::ServerSidePlayerData>,
    tick: usize,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Server {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Server init");

        Self {
            server: None,
            player_data: HashMap::new(),
            tick: 0,
            base,
        }
    }

    fn ready(&mut self) {
        self.server = Some(server::Server::new());
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();

        godot_print!("Server is ready! (Socket bound!)");
    }

    // The server is limited to 10FPS
    fn process(&mut self, delta: f64) {
        let start = Instant::now();

        // Update server
        self.update_server(delta);
        let event_cnt = self.handle_events();
        let msg_cnt = self.handle_messages();
        let msg_cnt = self.process_messages();

        // tracing::debug!("Handling networking took: {:?}", start.elapsed());

        self.tick += 1;
    }
}

#[godot_api]
impl Server {
    fn update_server(&mut self, delta: f64) {
        // Update server every tick
        self.server
            .as_mut()
            .unwrap()
            .update(Duration::from_secs_f64(delta))
            .map_err(|e| godot_error!("Error during server update: {:?}", e))
            .ok();
    }

    fn handle_events(&mut self) -> usize {
        let mut event_cnt = 0;
        // Handle server events
        self.server
            .as_mut()
            .unwrap()
            .get_events()
            .iter()
            .for_each(|event| {
                event_cnt += 1;
                match event {
                    ServerEvent::ClientConnected { client_id } => {
                        tracing::info!("Client {} connected", client_id);

                        let player_res = load::<PackedScene>("res://Scenes/Player.tscn");
                        let player_obj = player_res.instantiate_as::<CharacterBody2D>();
                        let player_data = common::ServerSidePlayerData::new(player_obj);

                        self.base_mut().add_child(&player_data.player);
                        self.player_data.insert(*client_id, player_data);
                    }
                    ServerEvent::ClientDisconnected { client_id, reason } => {
                        tracing::info!("Client {} disconnected, reason: {}", client_id, reason);
                        self.player_data.remove(client_id);
                    }
                }
            });

        event_cnt
    }

    fn handle_messages(&mut self) -> usize {
        // Get new messages
        self.server
            .as_mut()
            .unwrap()
            .get_messages(&mut self.player_data)
    }

    fn process_messages(&mut self) -> usize {
        // It is assumed the messages are ordered in the buffer!!!
        let mut msg_cnt = 0;

        for (client_id, data) in self.player_data.iter_mut() {
            let mut validated_client_messages = Vec::with_capacity(common::BUFFER_CAPACITY);
            // godot_print!("Processing client {}", client_id);
            for msg in data.messages.iter_mut() {
                // TODOs:
                // Check if every message arrived (E.G.: Send tick count from client side and check here if (cur_msg.tick - prev_msg.tick) == 1)
                // Check if messages are ordered
                // Handle player data on server side? (E.G.: speed)
                // Handle f32/f64 mismatch (Do not send f64 if it is not necessary - Vector2 calculates with f32s)
                // If a message (or multiple) is lost "guess it"
                // Handle "unprocessed" messages (it is simply cleared at the end)
                // Actually validate client messages (validate timestamps?)
                // Calculate + log and/or visualize network stats on both sides (ping, lost packets, bandwidth etc. <- check if Renet can provide those)
                // Handle client data correctly (not just assume player starts from pos (0, 0))
                // Spawn player on a server generated pos
                // Actually instantiate and move a player object on server side aswell
                if let common::Action::Movement(movement) = msg {
                    if (movement.input.0, movement.input.1) != (0, 0)
                        && movement.state == common::ActionState::SentByClient
                    {
                        let offset = Vector2::new(movement.input.0 as f32, movement.input.1 as f32)
                            .normalized()
                            * 100.0
                            * movement.delta as f32;

                        data.player.move_and_collide(offset);

                        tracing::debug!(
                            "{:?} ({:?}, {}, {})",
                            data.player.get_position(),
                            movement.input,
                            100.0,
                            movement.delta
                        );

                        movement.state = common::ActionState::ValidatedByServer;
                        validated_client_messages.push(msg.clone()); // TODO?
                    }
                }
                msg_cnt += 1;
            }

            // TODO: "Validated" messages are sent back to client but unhandled on client side
            self.server
                .as_mut()
                .unwrap()
                .send_frame(*client_id, validated_client_messages);

            // This kind of iteration will be needed (uncomment itertools import if needed)
            // for (prev, next) in messages.iter().tuple_windows() {}
        }

        msg_cnt
    }
}
