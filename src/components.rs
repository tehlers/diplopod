use bevy::prelude::*;

pub struct DiplopodHead {
    pub direction: Vec2,
}

pub struct DiplopodSegment;

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

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConsumablePosition {
    pub x: i32,
    pub y: i32,
}

pub struct Food;
