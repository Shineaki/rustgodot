use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;
use ringbuffer::{AllocRingBuffer, RingBuffer};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    pub base: Base<CharacterBody2D>,
    pub speed: f64,
    pub input: (i8, i8),
    pub delta: f64,
    pub tick: usize,
    pub animator: Option<Gd<AnimatedSprite2D>>,
    pub actions: AllocRingBuffer<common::Action>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            speed: 100.0,
            input: (0, 0),
            delta: 0.0,
            tick: 0,
            animator: None,
            actions: AllocRingBuffer::new(common::BUFFER_CAPACITY),
        }
    }

    fn ready(&mut self) {
        self.animator = Some(self.base().get_node_as::<AnimatedSprite2D>("Animator"));
    }

    fn physics_process(&mut self, delta: f64) {
        self.handle_input();

        let animator = self.animator.as_mut().expect("Animator node not found!");

        // Update player position
        if self.input == (0, 0) {
            animator.set_animation("Idle"); // TODO Enum
        } else {
            animator.set_animation("Run");
            animator.set_flip_h(self.input.0 <= 0);

            let offset = Vector2::new(self.input.0 as f32, self.input.1 as f32).normalized()
                * self.speed as f32
                * delta as f32;

            self.base_mut().move_and_collide(offset);

            self.actions
                .push(common::Action::Movement(common::Movement::new(
                    self.tick, delta, self.input,
                )));

            tracing::debug!(
                "{:?} ({:?}, {}, {})",
                self.base().get_position(),
                self.input,
                self.speed,
                delta
            );
        }

        self.delta = delta;
        self.tick += 1;
    }
}

#[godot_api]
impl Player {
    fn handle_input(&mut self) {
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

        self.input = (input_x, input_y)
    }
}
