use bevy::prelude::*;

use crate::prelude::*;

pub const MAX_X: f32 = 1920.0;
pub const MAX_Y: f32 = 1200.0;
pub const TILE_SIZE: f32 = MAX_X / ARENA_WIDTH as f32;
pub const UPPER_LEFT: Vec2 = Vec2::new(
    (MAX_X - (ARENA_WIDTH - 1) as f32 * TILE_SIZE) / 2.,
    (MAX_Y - (ARENA_HEIGHT - 1) as f32 * TILE_SIZE) / 2.,
);
