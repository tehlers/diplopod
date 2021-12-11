use crate::components::Size;
use crate::components::*;
use crate::events::SpawnConsumables;
use crate::prelude::{
    AMOUNT_OF_FOOD, AMOUNT_OF_POISON, ARENA_HEIGHT, ARENA_WIDTH, SPECIAL_SPAWN_INTERVAL,
};
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

fn spawn_random_antidote(
    commands: &mut Commands,
    materials: &Res<Materials>,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.antidote_material.clone(),
                ..Default::default()
            })
            .insert(Antidote)
            .insert(pos)
            .insert(Size::square(2.0));
        free_consumable_positions.remove(&pos);
    }
}

pub fn spawn_consumables(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_consumables_reader: EventReader<SpawnConsumables>,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    consumable_positions: Query<&ConsumablePosition>,
    superfood: Query<Entity, With<Superfood>>,
    antidotes: Query<Entity, With<Antidote>>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
) {
    if let Some(spawn_event) = spawn_consumables_reader.iter().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .map(|p| p.to_consumable_position())
            .collect::<Vec<ConsumablePosition>>();

        let mut position_candidates = free_consumable_positions.clone();
        position_candidates.remove_all(&segment_positions);

        if spawn_event.regular {
            spawn_random_food(
                1,
                &mut commands,
                &materials,
                &mut position_candidates,
                &mut free_consumable_positions,
            );

            spawn_random_poison(
                1,
                &mut commands,
                &materials,
                &mut position_candidates,
                &mut free_consumable_positions,
            );
        }

        let new_size = segments.0.len() as u32 + spawn_event.new_segments as u32;
        if new_size - last_special_spawn.0 > SPECIAL_SPAWN_INTERVAL {
            last_special_spawn.0 = (new_size / SPECIAL_SPAWN_INTERVAL) * SPECIAL_SPAWN_INTERVAL;

            for ent in superfood.iter() {
                let position = consumable_positions.get(ent).unwrap();
                free_consumable_positions.positions.push(position.clone());
                commands.entity(ent).despawn();
            }
            free_consumable_positions.shuffle();

            if last_special_spawn.0 % (SPECIAL_SPAWN_INTERVAL * 2) == 0 {
                for ent in antidotes.iter() {
                    let position = consumable_positions.get(ent).unwrap();
                    free_consumable_positions.positions.push(position.clone());
                    commands.entity(ent).despawn();
                }
                free_consumable_positions.shuffle();

                spawn_random_antidote(
                    &mut commands,
                    &materials,
                    &mut position_candidates,
                    &mut free_consumable_positions,
                );
            }

            spawn_random_superfood(
                &mut commands,
                &materials,
                &mut position_candidates,
                &mut free_consumable_positions,
            );
        }
    }
}
