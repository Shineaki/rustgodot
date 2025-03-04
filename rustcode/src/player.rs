use std::time::Duration;
use std::time::SystemTime;

use bincode::Options;
use godot::classes::AnimatedSprite2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::prelude::*;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use tracing_subscriber::EnvFilter;

const BUFFER_CAPACITY: usize = 100;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    client: client::Client,
    base: Base<CharacterBody2D>,

    cur_ts: f64,
    prev_ts: f64,

    facing_right: bool,
    animation_state: String, // TODO: replace with enum

    tick_cntr: usize,
    actions: AllocRingBuffer<common::Message>, // When dropping the last element it should be checked it is validated
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Client init");

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();

        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        Self {
            speed: 100.0,
            client: client::Client::new(),
            base,
            cur_ts: ts,
            prev_ts: ts,
            facing_right: true,
            animation_state: "Idle".to_string(),
            tick_cntr: 0,
            actions: AllocRingBuffer::new(BUFFER_CAPACITY),
        }
    }

    fn ready(&mut self) {
        godot_print!("Client ready");
    }

    fn process(&mut self, delta: f64) {
        let mut _animator = self.base().get_node_as::<AnimatedSprite2D>("Animator");
        _animator.set_animation(&self.animation_state);
        _animator.set_flip_h(!self.facing_right);

        self.client
            .update(Duration::from_secs_f64(delta))
            .map_err(|e| godot_error!("Error during client update: {:?}", e))
            .ok();

        // Send new actions to server
        if self.tick_cntr % 6 == 0 {
            let mut messages = vec![];
            for action in self.actions.iter_mut() {
                if action.state == common::MessageState::Created {
                    action.state = common::MessageState::Sent;
                    messages.push(action.clone()) // TODO
                }
            }

            if !messages.is_empty() {
                let frame = common::Frame::new(messages);
                let payload = bincode::options().serialize(&frame).unwrap();
                self.client.send(payload);
            }
        }

        // TODO: handle messages sent by server
        // TODO: validate messages (self.actions)

        self.prev_ts = self.cur_ts;
        self.tick_cntr += 1;
    }

    fn physics_process(&mut self, _delta: f64) {
        // let dt = self.get_dt_from_timestamp(); TODO: discuss
        let (input_x, input_y) = self.handle_input();

        // Update player position
        if (input_x, input_y) == (0, 0) {
            self.animation_state = "Idle".to_string();
        } else {
            self.animation_state = "Run".to_string();
            if self.facing_right && input_x < 0 {
                self.facing_right = false;
            } else if !self.facing_right && input_x > 0 {
                self.facing_right = true;
            }

            let offset = Vector2::new(input_x as f32, input_y as f32).normalized()
                * self.speed as f32
                * _delta as f32;

            let prev_pos = self.base().get_position();
            self.base_mut().move_and_collide(offset);
            let cur_pos = self.base().get_position();

            godot_print!("{:?}", cur_pos);

            // TODO: handle client ID
            self.actions.push(common::Message::new(
                412,
                common::ActionType::Movement(common::Movement::new(
                    (input_x, input_y),
                    self.cur_ts,
                    self.prev_ts,
                    cur_pos,
                    prev_pos,
                )),
            ));
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

    // fn get_dt_from_timestamp(&mut self) -> f64 {
    //     self.cur_ts = SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs_f64();

    //     self.cur_ts - self.prev_ts
    // }
}
