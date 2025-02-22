use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use crate::prelude::CONSUMABLE_SCALE_FACTOR;

use super::{
    DiplopodSegments,
    graphics::{MAX_X, MAX_Y, TILE_SIZE, UPPER_LEFT},
};

#[derive(Component)]
pub struct DiplopodHead {
    pub direction: Vec2,
    pub immunity: Timer,
}

impl Default for DiplopodHead {
    fn default() -> Self {
        Self {
            direction: Vec2::ZERO,
            immunity: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

#[derive(Component)]
#[component(on_add=on_add_diplopod_segment)]
pub struct DiplopodSegment;

fn on_add_diplopod_segment(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.resource_mut::<DiplopodSegments>().0.push(entity);
}

#[derive(Component, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DiplopodPosition {
    pub x: i32,
    pub y: i32,
}

impl From<DiplopodPosition> for Transform {
    fn from(position: DiplopodPosition) -> Self {
        Transform::from_xyz(
            position.x as f32 * TILE_SIZE + UPPER_LEFT.x - MAX_X / 2.,
            position.y as f32 * TILE_SIZE + UPPER_LEFT.y - MAX_Y / 2.,
            0.0,
        )
    }
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

#[derive(Component)]
pub enum Obstacle {
    Food,
    Poison,
    Superfood,
    Antidote,
    Wall,
}

#[derive(Component)]
pub struct Superfood;

#[derive(Component)]
pub struct Antidote;

#[derive(Component)]
pub struct FadingText(pub f32);

#[derive(Component)]
pub struct AntidoteSound;
