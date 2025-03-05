use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use bincode::Options;
use client::Client;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;
use godot_tokio::AsyncRuntime;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use tracing_subscriber::EnvFilter;

const BUFFER_CAPACITY: usize = 32;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    base: Base<CharacterBody2D>,

    cur_ts: f64,
    prev_ts: f64,

    facing_right: bool,
    animation_state: String, // TODO: replace with enum

    tick: usize,

    client: Arc<Mutex<client::Client>>,
    actions: AllocRingBuffer<common::Action>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Client init");

        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        Self {
            speed: 100.0,
            base,
            cur_ts: ts,
            prev_ts: ts,
            facing_right: true,
            animation_state: "Idle".to_string(),
            tick: 0,
            client: Arc::new(Mutex::new(Client::new())),
            actions: AllocRingBuffer::new(BUFFER_CAPACITY),
        }
    }

    fn ready(&mut self) {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();
    }

    fn process(&mut self, delta: f64) {
        let mut _animator = self.base().get_node_as::<AnimatedSprite2D>("Animator");
        _animator.set_animation(&self.animation_state);
        _animator.set_flip_h(!self.facing_right);

        // Send new actions to server
        if self.tick % 6 == 0 {
            // Collect unprocessed messages
            let mut actions = vec![];
            for action in self.actions.iter_mut() {
                match action {
                    common::Action::Movement(movement) => {
                        if movement.state == common::ActionState::New {
                            movement.state = common::ActionState::SentByClient;
                            actions.push(action.clone()) // TODO
                        }
                    }
                }
            }

            // if !actions.is_empty() {
            let client = self.client.clone();
            AsyncRuntime::spawn(async move {
                let start = Instant::now();
                match client.try_lock() {
                    Ok(mut client) => {
                        // Update client
                        client
                            .update(Duration::from_secs_f64(6.0 * delta))
                            // In AsyncRuntime scopes, godo_print/error/etc. does not work!
                            .map_err(|e| tracing::error!("Error during client update: {:?}", e))
                            .ok();

                        // Send data to server
                        let frame = common::Frame::new(actions);
                        let payload = bincode::options().serialize(&frame).unwrap();
                        client.send(payload);

                        // Get data from server
                        let new_frame = client.get_messages();

                        tracing::debug!("Handling networking took: {:?}\n{:?}", start.elapsed(), new_frame);
                    }
                    Err(_) => tracing::warn!("Networking seem to be slow, change something?!"),
                }
            });
            // }
        }

        // TODO: handle messages sent by server
        // TODO: validate messages (self.actions)

        self.prev_ts = self.cur_ts;
        self.tick += 1;
    }

    fn physics_process(&mut self, delta: f64) {
        let (input_x, input_y) = self.handle_input();

        // Update player position
        if (input_x, input_y) == (0, 0) {
            self.animation_state = "Idle".to_string();
        } else {
            self.animation_state = "Run".to_string();
            self.facing_right = input_x > 0;

            let offset = Vector2::new(input_x as f32, input_y as f32).normalized()
                * self.speed as f32
                * delta as f32;

            self.base_mut().move_and_collide(offset);

            tracing::debug!("{:.4}: {:?}", delta, self.base().get_position());

            // TODO: handle client ID
            self.actions
                .push(common::Action::Movement(common::Movement::new(
                    self.tick,
                    delta,
                    (input_x, input_y),
                )));
        }
    }
}

#[godot_api]
impl Player {
    fn handle_input(&mut self) -> (i8, i8) {
        let mut input_x = 0;
        let mut input_y = 0;
        let input = Input::singleton();

        if input.is_action_pressed("ui_left") {
            input_x -= 1;
        }

        if input.is_action_pressed("ui_right") {
            input_x += 1;
        }

        if input.is_action_pressed("ui_up") {
            input_y -= 1;
        }

        if input.is_action_pressed("ui_down") {
            input_y += 1;
        }

        (input_x, input_y)
    }
}
