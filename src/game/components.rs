use bevy::prelude::*;

use crate::{MAX_X, MAX_Y};

use super::{CONSUMABLE_SCALE_FACTOR, TILE_SIZE, UPPER_LEFT};

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<Position> for Transform {
    fn from(position: Position) -> Self {
        Transform::from_xyz(
            position.x as f32 * TILE_SIZE * CONSUMABLE_SCALE_FACTOR as f32 + UPPER_LEFT.x
                - MAX_X / 2.
                + TILE_SIZE / 2.,
            position.y as f32 * TILE_SIZE * CONSUMABLE_SCALE_FACTOR as f32 + UPPER_LEFT.y
                - MAX_Y / 2.
                + TILE_SIZE / 2.,
            1.0,
        )
    }
}

impl From<Transform> for Position {
    fn from(transform: Transform) -> Self {
        Position {
            x: ((transform.translation.x - UPPER_LEFT.x + MAX_X / 2. - TILE_SIZE / 2.)
                / (TILE_SIZE * CONSUMABLE_SCALE_FACTOR as f32))
                .round() as i32,
            y: ((transform.translation.y - UPPER_LEFT.y + MAX_Y / 2. - TILE_SIZE / 2.)
                / (TILE_SIZE * CONSUMABLE_SCALE_FACTOR as f32))
                .round() as i32,
        }
    }
}

#[derive(Component)]
pub enum Obstacle {
    Food,
    Poison,
    Superfood,
    Antidote,
    Wall,
}
