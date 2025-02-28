use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    thread,
    time::{Duration, SystemTime},
};

use bincode::Options;
use common::Message;
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};
use renet_visualizer::RenetClientVisualizer;
use tracing_subscriber::EnvFilter;

pub struct Client {
    pub client: RenetClient,
    pub transport: NetcodeClientTransport,
    pub visualizer: RenetClientVisualizer<240>,
}

impl Client {
    pub fn new() -> Self {
        let client_socket =
            UdpSocket::bind(format!("127.0.0.1:{}", 0)).expect("Could not bind socket!");
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let client: RenetClient = RenetClient::new(ConnectionConfig::default());
        let transport = NetcodeClientTransport::new(
            current_time,
            ClientAuthentication::Unsecure {
                protocol_id: common::PROTOCOL_ID,
                client_id: 412,
                server_addr: SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    common::PORT as u16,
                ),
                user_data: None,
            },
            client_socket,
        )
        .unwrap();

        Self {
            client,
            transport,
            visualizer: RenetClientVisualizer::default(),
        }
    }

    pub fn update(&mut self, duration: Duration) -> Result<(), io::Error> {
        self.client.update(duration);
        self.transport.update(duration, &mut self.client).unwrap();
        // self.visualizer.u(&self.client);

        Ok(())
    }

    pub fn send(&mut self, payload: Vec<u8>) {
        self.client
            .send_message(DefaultChannel::ReliableOrdered, payload);

        if self.client.is_connected() {
            self.transport.send_packets(&mut self.client).unwrap();
        }
    }
}
