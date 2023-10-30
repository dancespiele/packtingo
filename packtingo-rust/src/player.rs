use godot::engine::{CharacterBody3D, CharacterBody3DVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    speed: f32,
    target_velocity: Vector3,
    initial_position: Vector3,

    #[base]
    bar: Base<CharacterBody3D>,
}

#[godot_api]
impl CharacterBody3DVirtual for Player {
    fn init(bar: Base<CharacterBody3D>) -> Self {
        godot_print!("Initialize player!");

        Self {
            speed: 150.0,
            initial_position: Vector3::ZERO,
            target_velocity: Vector3::ZERO,
            bar,
        }
    }

    fn ready(&mut self) {
        self.initial_position = self.bar.get_position();
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let mut direction = Vector3::ZERO;

        let mut current_position = self.bar.get_position();

        if self.initial_position.x != current_position.x {
            current_position.x = self.initial_position.x;

            self.bar.set_position(current_position);
        }

        if Input::is_action_pressed(&input, "move_right".into()) {
            direction.z += 1.0;
        }

        if Input::is_action_pressed(&input, "move_left".into()) {
            direction.z -= 1.0;
        }

        self.target_velocity.z = direction.z * self.speed * delta as f32;

        self.bar.set_velocity(self.target_velocity);
        self.bar.move_and_slide();
    }
}
