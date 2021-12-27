use crate::components::Size;
use crate::components::*;
use crate::events::SpawnConsumables;
use crate::prelude::{
    AMOUNT_OF_FOOD, AMOUNT_OF_POISON, ANTIDOTE_COLOR, ARENA_HEIGHT, ARENA_WIDTH, DIPLOPOD_COLOR,
    FOOD_COLOR, POISON_COLOR, SPECIAL_SPAWN_INTERVAL, SUPERFOOD_COLOR,
};
use crate::resources::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn init_diplopod(mut commands: Commands, mut segments: ResMut<DiplopodSegments>) {
    spawn_diplopod(&mut commands, &mut segments);
}

pub fn spawn_diplopod(commands: &mut Commands, segments: &mut ResMut<DiplopodSegments>) {
    segments.0 = vec![commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: DIPLOPOD_COLOR,
                ..Default::default()
            },
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

pub fn spawn_segment(commands: &mut Commands, color: Color, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DiplopodSegment)
        .insert(position)
        .insert(Size::square(1.0))
        .id()
}

pub fn init_food(
    mut commands: Commands,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    consumable_radius: Res<ConsumableRadius>,
) {
    spawn_food(
        &mut commands,
        &mut free_consumable_positions,
        &consumable_radius,
    );
}

pub fn spawn_food(
    commands: &mut Commands,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    consumable_radius: &Res<ConsumableRadius>,
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
        &mut position_candidates,
        free_consumable_positions,
        &consumable_radius,
    );
}

fn spawn_random_food(
    amount: u32,
    commands: &mut Commands,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    consumable_radius: &Res<ConsumableRadius>,
) {
    let shape = shapes::Circle {
        radius: consumable_radius.0,
        center: Vec2::new(0., 0.),
    };

    for _ in 0..amount {
        match position_candidates.positions.pop() {
            None => break,
            Some(pos) => {
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(FOOD_COLOR),
                            outline_mode: StrokeMode::color(FOOD_COLOR),
                        },
                        Transform::default(),
                    ))
                    .insert(Food)
                    .insert(pos);
                free_consumable_positions.remove(&pos);
            }
        }
    }
}

pub fn init_poison(
    mut commands: Commands,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    consumable_radius: Res<ConsumableRadius>,
) {
    spawn_poison(
        &mut commands,
        &mut free_consumable_positions,
        &consumable_radius,
    );
}

pub fn spawn_poison(
    commands: &mut Commands,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    consumable_radius: &Res<ConsumableRadius>,
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
        &mut position_candidates,
        free_consumable_positions,
        consumable_radius,
    );
}

fn spawn_random_poison(
    amount: u32,
    commands: &mut Commands,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    consumable_radius: &Res<ConsumableRadius>,
) {
    let shape = shapes::Circle {
        radius: consumable_radius.0,
        center: Vec2::new(0., 0.),
    };

    for _ in 0..amount {
        match position_candidates.positions.pop() {
            None => break,
            Some(pos) => {
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(POISON_COLOR),
                            outline_mode: StrokeMode::color(POISON_COLOR),
                        },
                        Transform::default(),
                    ))
                    .insert(Poison)
                    .insert(pos);
                free_consumable_positions.remove(&pos);
            }
        }
    }
}

fn spawn_random_superfood(
    commands: &mut Commands,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SUPERFOOD_COLOR,
                    ..Default::default()
                },
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
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: ANTIDOTE_COLOR,
                    ..Default::default()
                },
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
    mut positions: Query<&mut Position>,
    consumable_positions: Query<&ConsumablePosition>,
    superfood: Query<Entity, With<Superfood>>,
    antidotes: Query<Entity, With<Antidote>>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    consumable_radius: Res<ConsumableRadius>,
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
                &mut position_candidates,
                &mut free_consumable_positions,
                &consumable_radius,
            );

            spawn_random_poison(
                1,
                &mut commands,
                &mut position_candidates,
                &mut free_consumable_positions,
                &consumable_radius,
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
                    &mut position_candidates,
                    &mut free_consumable_positions,
                );
            }

            spawn_random_superfood(
                &mut commands,
                &mut position_candidates,
                &mut free_consumable_positions,
            );
        }
    }
}
