use bevy::prelude::*;

use crate::prelude::CONSUMABLE_SCALE_FACTOR;

#[derive(Component)]
pub struct DiplopodHead {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct DiplopodSegment;

#[derive(Component, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DiplopodPosition {
    pub x: i32,
    pub y: i32,
}

impl DiplopodPosition {
    pub fn to_position(self) -> Position {
        Position {
            x: self.x / CONSUMABLE_SCALE_FACTOR,
            y: self.y / CONSUMABLE_SCALE_FACTOR,
        }
    }
}

#[derive(Component, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Poison;

#[derive(Component)]
pub struct Superfood;

#[derive(Component)]
pub struct Antidote;

#[derive(Component)]
pub struct FadingText(pub f32);

#[derive(Component)]
pub struct AntidoteSound;
