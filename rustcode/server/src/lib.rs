use std::{
    collections::HashMap,
    net::UdpSocket,
    time::{Duration, SystemTime},
};

use bincode::Options;
use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{
    NetcodeServerTransport, NetcodeTransportError, ServerAuthentication, ServerConfig,
};
use renet_visualizer::RenetServerVisualizer;

pub struct Server {
    pub server: RenetServer,
    pub transport: NetcodeServerTransport,
    pub visualizer: RenetServerVisualizer<240>,
}

impl Server {
    pub fn new() -> Self {
        let socket =
            // UdpSocket::bind(format!("0.0.0.0:{}", common::PORT)).expect("Could not bind socket!");
            UdpSocket::bind(format!("127.0.0.1:{}", common::PORT)).expect("Could not bind socket!");
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

        let server: RenetServer = RenetServer::new(ConnectionConfig::default());
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

        Self {
            server,
            transport,
            visualizer: RenetServerVisualizer::default(),
        }
    }

    pub fn update(&mut self, duration: Duration) -> Result<(), NetcodeTransportError> {
        self.server.update(duration);
        self.transport.update(duration, &mut self.server)?;
        self.visualizer.update(&self.server);

        Ok(())
    }

    pub fn get_events(&mut self) -> Vec<ServerEvent> {
        self.server
            .get_event()
            .into_iter()
            .map(|event| event)
            .collect()
    }

    pub fn get_messages(&mut self, player_data: &mut HashMap<u64, Vec<common::Message>>) -> usize {
        let mut msg_cnt = 0;

        self.server.clients_id().iter().for_each(|client_id| {
            if let Some(player_frames) = player_data.get_mut(client_id) {
                // Only get frames from a client if it is already went through the connection event
                player_frames.extend(self.get_client_frames(*client_id));
                msg_cnt += 1;
            }
        });

        msg_cnt
    }

    fn get_client_frames(&mut self, client_id: u64) -> Vec<common::Message> {
        let mut messages = vec![];

        while let Some(raw_message) = self
            .server
            .receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            match bincode::options().deserialize::<common::Frame>(&raw_message) {
                Ok(frame) => messages.extend(frame.messages),
                Err(_) => tracing::warn!("Could not deserialize message!"),
            }
        }

        messages
    }

    pub fn send_frame(&mut self, client_id: u64, messages: Vec<common::Message>) {
        let frame = common::Frame::new(messages);
        let payload = bincode::options().serialize(&frame).unwrap();
        self.server
            .send_message(client_id, DefaultChannel::ReliableOrdered, payload);

        self.transport.send_packets(&mut self.server);
    }
}
