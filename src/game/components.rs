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

#[derive(Component)]
pub struct Superfood;

#[derive(Component)]
pub struct Antidote;

#[derive(Component)]
pub struct AntidoteSound;
