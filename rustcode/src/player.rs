use std::time::Duration;

use bincode::Options;
use godot::classes::ISprite2D;
use godot::classes::Sprite2D;
use godot::prelude::*;
use tracing_subscriber::EnvFilter;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,
    input: (i8, i8),

    client: client::Client,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            input: (0, 0),
            client: client::Client::new(),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        self.client
            .update(Duration::from_secs_f64(delta))
            .map_err(|e| godot_error!("Error during client update: {:?}", e))
            .ok();

        if self.input != (0, 0) {
            // Send InputChanged event to server!
            let msg = common::Message::new(
                1,
                common::MessageType::InputChanged,
                common::InputChanged::new(self.input),
            );
            let payload = bincode::options().serialize(&msg).unwrap();

            self.client.send(payload);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.handle_input();

        if self.input != (0, 0) {
            // Update player position
            let velocity = Vector2::new(self.input.0 as f32, self.input.1 as f32).normalized()
                * self.speed as f32;

            godot_print!("{:?}", velocity);

            self.base_mut().translate(velocity * delta as f32);
        }
    }
}

#[godot_api]
impl Player {
    fn handle_input(&mut self) {
        self.input = (0, 0);
        let input = Input::singleton();

        if input.is_action_pressed("ui_left") {
            self.input.0 -= 1;
        }

        if input.is_action_pressed("ui_right") {
            self.input.0 += 1;
        }

        if input.is_action_pressed("ui_up") {
            self.input.1 -= 1;
        }

        if input.is_action_pressed("ui_down") {
            self.input.1 += 1;
        }
    }
}
