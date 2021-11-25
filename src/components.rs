use bevy::prelude::*;

pub struct DiplopodHead {
    pub direction: Vec2,
}
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
