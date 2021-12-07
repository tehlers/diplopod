use crate::components::Size;
use crate::components::*;
use crate::events::{SpawnFood, SpawnPoison, SpawnSuperfood};
use crate::prelude::{AMOUNT_OF_FOOD, AMOUNT_OF_POISON, ARENA_HEIGHT, ARENA_WIDTH};
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

pub fn init_food(
    mut commands: Commands,
    materials: Res<Materials>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    spawn_food(&mut commands, &materials, &mut free_consumable_positions);
}

pub fn spawn_food(
    commands: &mut Commands,
    materials: &Res<Materials>,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    let segment_positions = vec![Position {
        x: ARENA_WIDTH / 2,
        y: ARENA_HEIGHT / 2,
    }
    .to_consumable_position()];

    let mut position_candidates = free_consumable_positions.clone();
    position_candidates.remove_all(&segment_positions);

    spawn_random_food(
        AMOUNT_OF_FOOD,
        commands,
        materials,
        &mut position_candidates,
        free_consumable_positions,
    );
}

pub fn spawn_new_food(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_food_reader: EventReader<SpawnFood>,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    if spawn_food_reader.iter().next().is_some() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .map(|p| p.to_consumable_position())
            .collect::<Vec<ConsumablePosition>>();

        let mut position_candidates = free_consumable_positions.clone();
        position_candidates.remove_all(&segment_positions);

        spawn_random_food(
            1,
            &mut commands,
            &materials,
            &mut position_candidates,
            &mut free_consumable_positions,
        );
    }
}

fn spawn_random_food(
    amount: u32,
    commands: &mut Commands,
    materials: &Res<Materials>,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    for _ in 0..amount {
        match position_candidates.positions.pop() {
            None => break,
            Some(pos) => {
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
    }
}

pub fn init_poison(
    mut commands: Commands,
    materials: Res<Materials>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    spawn_poison(&mut commands, &materials, &mut free_consumable_positions);
}

pub fn spawn_poison(
    commands: &mut Commands,
    materials: &Res<Materials>,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    let segment_positions = vec![Position {
        x: ARENA_WIDTH / 2,
        y: ARENA_HEIGHT / 2,
    }
    .to_consumable_position()];

    let mut position_candidates = free_consumable_positions.clone();
    position_candidates.remove_all(&segment_positions);

    spawn_random_poison(
        AMOUNT_OF_POISON,
        commands,
        materials,
        &mut position_candidates,
        free_consumable_positions,
    );
}

pub fn spawn_new_poison(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_poison_reader: EventReader<SpawnPoison>,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    if spawn_poison_reader.iter().next().is_some() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .map(|p| p.to_consumable_position())
            .collect::<Vec<ConsumablePosition>>();

        let mut position_candidates = free_consumable_positions.clone();
        position_candidates.remove_all(&segment_positions);

        spawn_random_poison(
            1,
            &mut commands,
            &materials,
            &mut position_candidates,
            &mut free_consumable_positions,
        );
    }
}

fn spawn_random_poison(
    amount: u32,
    commands: &mut Commands,
    materials: &Res<Materials>,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    for _ in 0..amount {
        match position_candidates.positions.pop() {
            None => break,
            Some(pos) => {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.poison_material.clone(),
                        ..Default::default()
                    })
                    .insert(Poison)
                    .insert(pos)
                    .insert(Size::square(2.0));
                free_consumable_positions.remove(&pos);
            }
        }
    }
}

pub fn spawn_new_superfood(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_superfood_reader: EventReader<SpawnSuperfood>,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    if spawn_superfood_reader.iter().next().is_some() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .map(|p| p.to_consumable_position())
            .collect::<Vec<ConsumablePosition>>();

        let mut position_candidates = free_consumable_positions.clone();
        position_candidates.remove_all(&segment_positions);

        spawn_random_superfood(
            &mut commands,
            &materials,
            &mut position_candidates,
            &mut free_consumable_positions,
        );
    }
}

fn spawn_random_superfood(
    commands: &mut Commands,
    materials: &Res<Materials>,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.superfood_material.clone(),
                ..Default::default()
            })
            .insert(Superfood)
            .insert(pos)
            .insert(Size::square(2.0));
        free_consumable_positions.remove(&pos);
    }
}
