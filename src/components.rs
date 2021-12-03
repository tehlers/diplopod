use bevy::prelude::*;

use crate::prelude::CONSUMABLE_SCALE_FACTOR;

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

impl Position {
    pub fn to_consumable_position(&self) -> ConsumablePosition {
        ConsumablePosition {
            x: self.x / CONSUMABLE_SCALE_FACTOR,
            y: self.y / CONSUMABLE_SCALE_FACTOR,
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConsumablePosition {
    pub x: i32,
    pub y: i32,
}

pub struct Food;
