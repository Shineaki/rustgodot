// use std::str::FromStr;
// use std::time::Duration;

// use godot::classes::INode2D;
// use godot::classes::Node2D;
// use godot::prelude::*;
// use tracing_subscriber::EnvFilter;

// #[derive(GodotClass)]
// #[class(base=Node2D)]
// struct Client {
//     client: client::Client,
//     base: Base<Node2D>,
// }

// #[godot_api]
// impl INode2D for Client {
//     fn init(base: Base<Node2D>) -> Self {
//         tracing_subscriber::fmt()
//             .with_env_filter(EnvFilter::new("debug"))
//             .init();

//         Self {
//             client: client::Client::new(),
//             base,
//         }
//     }

//     fn physics_process(&mut self, delta: f64) {
//         // self.handle_input();

//         self.client
//             .update(Duration::from_secs_f64(delta))
//             .map_err(|e| godot_error!("Error during server update: {:?}", e))
//             .ok();
//     }
// }

// #[godot_api]
// impl Client {
//     fn handle_input(&self) {
//         // let input = Input::singleton();

//         // if input.is_action_pressed(StringName::from_str())
//     }
// }