use godot::classes::{INode2D, Node2D};
use godot::prelude::*;
use godot_tokio::AsyncRuntime;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
// use itertools::Itertools;
use renet::ServerEvent;
use tracing_subscriber::EnvFilter;

const BUFFER_CAPACITY: usize = 32;

// TODO: clock synchronization!

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Server {
    server: Option<Arc<Mutex<server::Server>>>,
    player_data: Arc<Mutex<HashMap<u64, Vec<common::Message>>>>,
    tick_cntr: usize,

    pos: Vector2,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Server {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Server init");

        Self {
            server: None,
            player_data: Arc::new(Mutex::new(HashMap::new())),
            tick_cntr: 0,
            pos: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        self.server = Some(Arc::new(Mutex::new(server::Server::new())));
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();
        godot_print!("Server is ready! (Socket bound!)");
    }

    // The server is limited to 10FPS
    fn process(&mut self, delta: f64) {
        let t0 = Instant::now();

        let server = self.server.clone();
        let player_data = self.player_data.clone();
        AsyncRuntime::spawn(async move {
            let start = Instant::now();
            // TODO: Server should be initialized at this point, but should be handled if not!
            match server.unwrap().try_lock() {
                Ok(mut server) => {
                    // Update server
                    Server::update_server(&mut server, delta);

                    let mut pd = player_data.lock().unwrap();

                    let event_cnt = Server::handle_events(&mut server, &mut pd);
                    
                    let msg_cnt = Server::handle_messages(&mut server, &mut pd);
                    let msg_cnt = Server::process_messages(&mut server, &mut pd);

                    tracing::debug!("Handling networking took: {:?}", start.elapsed());
                }
                Err(_) => tracing::warn!("Networking seem to be slow, change something?!"),
            }
        });

        self.tick_cntr += 1;
    }
}

#[godot_api]
impl Server {
    fn update_server(server: &mut server::Server, delta: f64) {
        // Update server every tick
        server
            .update(Duration::from_secs_f64(delta))
            .map_err(|e| godot_error!("Error during server update: {:?}", e))
            .ok();
    }

    fn handle_events(
        server: &mut server::Server,
        player_data: &mut HashMap<u64, Vec<common::Message>>,
    ) -> usize {
        let mut event_cnt = 0;
        // Handle server events
        server.get_events().iter().for_each(|event| {
            event_cnt += 1;
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    tracing::info!("Client {} connected", client_id);
                    player_data.insert(*client_id, Vec::with_capacity(BUFFER_CAPACITY));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    tracing::info!("Client {} disconnected, reason: {}", client_id, reason);
                    player_data.remove(client_id);
                }
            }
        });

        event_cnt
    }

    fn handle_messages(
        server: &mut server::Server,
        player_data: &mut HashMap<u64, Vec<common::Message>>,
    ) -> usize {
        // Get new messages
        server.get_messages(player_data)
    }

    fn process_messages(
        server: &mut server::Server,
        player_data: &mut HashMap<u64, Vec<common::Message>>,
    ) -> usize {
        // It is assumed the messages are ordered in the buffer!!!
        let mut msg_cnt = 0;

        for (client_id, messages) in player_data.iter_mut() {
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
                        // assert!(dt > 0.0); // TODO!!!!!!!!!!
                        let offset = Vector2::new(movement.input.0 as f32, movement.input.1 as f32)
                            .normalized()
                            * 100.0
                            * dt as f32;

                        // self.pos += offset;

                        msg.state = common::MessageState::ServerValidated;

                        validated_client_messages.push(msg.clone()); // TODO?

                        tracing::debug!("{}", offset);
                    }
                }
                msg_cnt += 1;
            }

            // TODO: "Validated" messages are sent back to client but unhandled on client side
            server.send_frame(*client_id, validated_client_messages);

            // This kind of iteration will be needed (uncomment itertools import if needed)
            // for (prev, next) in messages.iter().tuple_windows() {}

            messages.clear();
        }

        msg_cnt
    }
}
