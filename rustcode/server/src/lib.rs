use std::{
    collections::HashMap,
    io,
    net::UdpSocket,
    thread,
    time::{Duration, SystemTime},
};

use bincode::Options;
use renet::{ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use renet_visualizer::RenetServerVisualizer;
use tracing_subscriber::EnvFilter;

pub struct Server {
    pub server: RenetServer,
    pub transport: NetcodeServerTransport,
    pub players: HashMap<ClientId, String>,
    pub messages: Vec<common::Message>,
    pub visualizer: RenetServerVisualizer<240>,
}

impl Server {
    pub fn new() -> Self {
        let socket =
            UdpSocket::bind(format!("0.0.0.0:{}", common::PORT)).expect("Could not bind socket!");
            // UdpSocket::bind(format!("127.0.0.1:{}", common::PORT)).expect("Could not bind socket!");
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: common::PROTOCOL_ID,
            public_addresses: vec![socket.local_addr().expect("Could not get socet address!")],
            authentication: ServerAuthentication::Unsecure,
        };

        let players = HashMap::new();
        let server: RenetServer = RenetServer::new(ConnectionConfig::default());
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

        Self {
            server,
            transport,
            players,
            messages: vec![],
            visualizer: RenetServerVisualizer::default(),
        }
    }

    pub fn update(&mut self, duration: Duration) -> Result<(), io::Error> {
        tracing::debug!("tick");

        self.server.update(duration);
        self.transport.update(duration, &mut self.server).unwrap();
        self.visualizer.update(&self.server);

        while let Some(event) = self.server.get_event() {
            tracing::info!("[Event]: {:?}", event);

            match event {
                ServerEvent::ClientConnected { client_id } => {
                    // Handle user data
                    let user_data = self.transport.user_data(client_id).unwrap();
                    self.visualizer.add_client(client_id);

                    let username = common::Username::from_user_data(&user_data).0;
                    self.players.insert(client_id, username.clone());
                    tracing::info!("Players: {:?}", self.players);

                    // let message = bincode::options()
                    //     .serialize(&common::ServerMessages::ClientConnected {
                    //         client_id,
                    //         username,
                    //     })
                    //     .unwrap();

                    // send welcome message
                }
                ServerEvent::ClientDisconnected {
                    client_id,
                    reason: _,
                } => {
                    self.visualizer.remove_client(client_id);
                    self.players.remove(&client_id);
                }
            }
        }

        for client_id in self.server.clients_id() {
            for channel_id in 0..3 {
                // tracing::debug!("Checking client: {}", client_id);
                while let Some(message) = self.server.receive_message(client_id, channel_id) {
                    tracing::debug!("[{}][Raw Message]: {:?}", client_id, message);

                    if let Ok(message) = bincode::options().deserialize::<common::Message>(&message)
                    {
                        tracing::debug!("[{}][Message]: {:?}", client_id, message);
                    }
                }
            }
        }

        self.transport.send_packets(&mut self.server);

        Ok(())
    }
}
