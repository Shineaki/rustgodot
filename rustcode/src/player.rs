use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;
use ringbuffer::{AllocRingBuffer, RingBuffer};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    pub speed: f32,
    pub base: Base<CharacterBody2D>,
    pub tick: usize,
    pub facing_right: bool,
    pub animator: Option<Gd<AnimatedSprite2D>>,
    pub actions: AllocRingBuffer<common::Action>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 100.0,
            base,
            tick: 0,
            facing_right: true,
            animator: None,
            actions: AllocRingBuffer::new(common::BUFFER_CAPACITY),
        }
    }

    fn ready(&mut self) {
        self.animator = Some(self.base().get_node_as::<AnimatedSprite2D>("Animator"));
    }

    fn physics_process(&mut self, delta: f64) {
        let (input_x, input_y) = self.handle_input();

        let animator = self.animator.as_mut().expect("Animator node not found!");

        // Update player position
        if (input_x, input_y) == (0, 0) {
            animator.set_animation("Idle"); // TODO Enum
        } else {
            animator.set_animation("Run");
            if self.facing_right && input_x < 0 {
                self.facing_right = false;
            } else if !self.facing_right && input_x > 0 {
                self.facing_right = true;
            }

            animator.set_flip_h(!self.facing_right);

            // TODO: #Rust :)
            let speed = self.speed.clone();
            common::player_movement(&mut self.base_mut(), (input_x, input_y), speed, delta);

            self.actions
                .push(common::Action::Movement(common::Movement::new(
                    self.tick,
                    delta,
                    (input_x, input_y),
                    self.base().get_position(),
                )));

            tracing::debug!("{:?}", self.base().get_position());
        }

        self.tick += 1;
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
