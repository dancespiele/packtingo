use godot::engine::{StaticBody3D, StaticBody3DVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=StaticBody3D)]
struct Box {
    #[base]
    point: Base<StaticBody3D>,
}

#[godot_api]
impl StaticBody3DVirtual for Box {
    fn init(point: Base<StaticBody3D>) -> Self {
        Self { point }
    }
}
