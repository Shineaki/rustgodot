use std::{thread, time::Duration};

use server::Server;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .init();

    let mut server = Server::new();

    let duration = Duration::from_millis(1000);
    loop {
        match server.update(duration) {
            Ok(_) => (),
            Err(e) => tracing::error!("{:?}", e),
        }
        thread::sleep(duration);
    }
}
