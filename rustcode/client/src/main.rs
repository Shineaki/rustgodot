use std::{thread, time::Duration};

use client::Client;
use tracing_subscriber::EnvFilter;

fn main() {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .init();

    let mut client = Client::new();

    let duration = Duration::from_millis(2000);
    loop {
        match client.update(duration) {
            Ok(_) => (),
            Err(e) => tracing::error!("{:?}", e),
        }
        thread::sleep(duration);
    }
}