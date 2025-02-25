// use std::time::Duration;

// use godot::classes::INode2D;
// use godot::classes::Node2D;
// use godot::prelude::*;
// use tracing_subscriber::EnvFilter;

// #[derive(GodotClass)]
// #[class(base=Node2D)]
// struct Server {
//     server: server::Server,
//     base: Base<Node2D>,
// }

// #[godot_api]
// impl INode2D for Server {
//     fn init(base: Base<Node2D>) -> Self {
//         tracing_subscriber::fmt()
//             .with_env_filter(EnvFilter::new("debug"))
//             .init();

//         Self {
//             server: server::Server::new(),
//             base,
//         }
//     }

//     fn physics_process(&mut self, delta: f64) {
//         self.server
//             .update(Duration::from_secs_f64(delta))
//             .map_err(|e| godot_error!("Error during server update: {:?}", e))
//             .ok();
//     }
// }
