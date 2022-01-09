use bevy::prelude::*;

use crate::prelude::CONSUMABLE_SCALE_FACTOR;

#[derive(Component)]
pub struct DiplopodHead {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct DiplopodSegment;

#[derive(Component)]
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

#[derive(Component, Default, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Component, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConsumablePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Poison;

#[derive(Component)]
pub struct Superfood;

#[derive(Component)]
pub struct Antidote;
