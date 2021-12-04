use crate::components::Size;
use crate::components::*;
use crate::prelude::{AMOUNT_OF_FOOD, ARENA_HEIGHT, ARENA_WIDTH};
use crate::resources::*;
use bevy::prelude::*;

pub fn init_diplopod(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<DiplopodSegments>,
) {
    spawn_diplopod(&mut commands, &materials, &mut segments);
}

pub fn spawn_diplopod(
    commands: &mut Commands,
    materials: &Res<Materials>,
    segments: &mut ResMut<DiplopodSegments>,
) {
    segments.0 = vec![commands
        .spawn_bundle(SpriteBundle {
            material: materials.diplopod_material.clone(),
            ..Default::default()
        })
        .insert(DiplopodHead {
            direction: Vec2::ZERO,
        })
        .insert(DiplopodSegment)
        .insert(Position {
            x: ARENA_WIDTH / 2,
            y: ARENA_HEIGHT / 2,
        })
        .insert(Size::square(1.0))
        .id()];
}

pub fn spawn_segment(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            ..Default::default()
        })
        .insert(DiplopodSegment)
        .insert(position)
        .insert(Size::square(1.0))
        .id()
}

pub fn spawn_food(
    mut commands: Commands,
    materials: Res<Materials>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    let segment_positions = vec![Position {
        x: ARENA_WIDTH / 2,
        y: ARENA_HEIGHT / 2,
    }
    .to_consumable_position()];

    let mut position_candidates = free_consumable_positions.clone();
    position_candidates.remove_all(&segment_positions);

    for _ in 0..AMOUNT_OF_FOOD {
        let pos = position_candidates.positions.pop().unwrap();
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .insert(Food)
            .insert(pos)
            .insert(Size::square(2.0));
        free_consumable_positions.remove(&pos);
    }
}
