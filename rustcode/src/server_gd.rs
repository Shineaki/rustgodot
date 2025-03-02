use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use godot::classes::INode2D;
use godot::classes::Node2D;
use godot::prelude::*;
// use itertools::Itertools;
use renet::ServerEvent;
// use tracing_subscriber::EnvFilter;

const BUFFER_CAPACITY: usize = 100;

// TODO: clock synchronization!

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Server {
    server: server::Server,
    player_data: HashMap<u64, Vec<common::Message>>,
    tick_cntr: usize,

    pos: Vector2,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Server {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Server init");
        // tracing_subscriber::fmt()
        //     .with_env_filter(EnvFilter::new("debug"))
        //     .init();

        Self {
            server: server::Server::new(),
            player_data: HashMap::new(),
            tick_cntr: 0,
            pos: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Server ready");
    }

    fn process(&mut self, delta: f64) {
        let t0 = Instant::now();
        self.update_server(delta);

        // The server is limited to 10FPS
        if self.tick_cntr % 2 == 0 {
            let event_cnt = self.handle_events();
            let msg_cnt = self.handle_messages();

            // godot_print!(
            //     "[{} {:?}] Stored: {} events, {} message",
            //     self.tick_cntr,
            //     t0.elapsed(),
            //     event_cnt,
            //     msg_cnt
            // );
        } else {
            let msg_cnt = self.process_messages();

            // godot_print!(
            //     "[{} {:?}] Processed: {} messages",
            //     self.tick_cntr,
            //     t0.elapsed(),
            //     msg_cnt,
            // );
        }

        self.tick_cntr += 1;
    }
}

#[godot_api]
impl Server {
    fn update_server(&mut self, delta: f64) {
        // Update server every tick
        self.server
            .update(Duration::from_secs_f64(delta))
            .map_err(|e| godot_error!("Error during server update: {:?}", e))
            .ok();
    }

    fn handle_events(&mut self) -> usize {
        let mut event_cnt = 0;
        // Handle server events
        self.server.get_events().iter().for_each(|event| {
            event_cnt += 1;
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    godot_print!("Client {} connected", client_id);
                    self.player_data
                        .insert(*client_id, Vec::with_capacity(BUFFER_CAPACITY));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    godot_print!("Client {} disconnected, reason: {}", client_id, reason);
                    self.player_data.remove(client_id);
                }
            }
        });

        event_cnt
    }

    fn handle_messages(&mut self) -> usize {
        // Get new messages
        self.server.get_messages(&mut self.player_data)
    }

    fn process_messages(&mut self) -> usize {
        // It is assumed the messages are ordered in the buffer!!!
        let mut msg_cnt = 0;

        for (client_id, messages) in self.player_data.iter_mut() {
            let mut validated_client_messages = Vec::with_capacity(messages.len());
            // godot_print!("Processing client {}", client_id);
            for msg in messages.iter_mut() {
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
                if let common::ActionType::Movement(movement) = &msg.action {
                    if (movement.input.0, movement.input.1) != (0, 0) {
                        let dt = movement.cur_ts - movement.prev_ts;
                        assert!(dt > 0.0);
                        self.pos += Vector2::new(movement.input.0 as f32, movement.input.1 as f32)
                            .normalized()
                            * 100.0
                            * dt as f32;

                        msg.state = common::MessageState::ServerValidated;

                        validated_client_messages.push(msg.clone()); // TODO?

                        godot_print!("{}", self.pos);
                    }
                }
                msg_cnt += 1;
            }

            // TODO: "Validated" messages are sent back to client but unhandled on client side
            self.server
                .send_frame(*client_id, validated_client_messages);

            // This kind of iteration will be needed (uncomment itertools import if needed)
            // for (prev, next) in messages.iter().tuple_windows() {}

            messages.clear();
        }

        msg_cnt
    }
}
