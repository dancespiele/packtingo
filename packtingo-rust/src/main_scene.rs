use godot::prelude::*;
use godot::{
    engine::{Node3D, Node3DVirtual},
    prelude::godot_api,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
struct MainScene {
    #[base]
    _stage: Base<Node3D>,
}

#[godot_api]
impl Node3DVirtual for MainScene {
    fn init(stage: Base<Node3D>) -> Self {
        Self { _stage: stage }
    }
}
