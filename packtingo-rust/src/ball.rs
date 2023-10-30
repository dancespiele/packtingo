use godot::engine::{CharacterBody3D, CharacterBody3DVirtual};
use godot::prelude::utilities::randf_range;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Ball {
    target_velocity: Vector3,
    initial_position: Vector3,

    #[base]
    ball: Base<CharacterBody3D>,
}

#[godot_api]
impl CharacterBody3DVirtual for Ball {
    fn init(ball: Base<CharacterBody3D>) -> Self {
        Self {
            target_velocity: Vector3::ZERO,
            ball,
            initial_position: Vector3::ZERO,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        if Input::is_action_pressed(&input, "start".into()) && self.target_velocity == Vector3::ZERO
        {
            self.target_velocity.x = randf_range(1.0, 3.0) as f32 * delta as f32;
            self.target_velocity.z = randf_range(-1.0, 1.0) as f32 * delta as f32;
            self.initial_position = self.ball.get_position();

            self.ball.set_velocity(self.target_velocity)
        }

        if self.target_velocity != Vector3::ZERO {
            let current_velocity = self.ball.get_velocity();
            let collision_object_option = self.ball.move_and_collide(current_velocity);

            if let Some(collision_object) = collision_object_option {
                let mut bounce = current_velocity.bounce(collision_object.get_normal());

                bounce.x = bounce.x + randf_range(-0.9, 2.0) as f32 * delta as f32;

                self.ball.set_velocity(bounce)
            }
        }
    }
}

#[godot_api]
impl Ball {
    #[func]
    fn exit_screen(&mut self) {
        self.target_velocity = Vector3::ZERO;

        self.ball.set_velocity(self.target_velocity);
        self.ball.set_position(self.initial_position);
    }
}
