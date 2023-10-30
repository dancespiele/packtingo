use godot::prelude::*;

struct Packtingo;

mod ball;
mod box_point;
mod main_scene;
mod player;
mod wall;

#[gdextension]
unsafe impl ExtensionLibrary for Packtingo {}
