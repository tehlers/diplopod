use bevy::prelude::*;

pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
