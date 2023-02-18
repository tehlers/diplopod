use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::*;
use crate::events::*;
use crate::prelude::*;
use crate::resources::*;

pub fn init_diplopod(mut commands: Commands, mut segments: ResMut<DiplopodSegments>) {
    spawn_diplopod(&mut commands, &mut segments);
}

fn spawn_diplopod(commands: &mut Commands, segments: &mut ResMut<DiplopodSegments>) {
    segments.0 = vec![commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: DIPLOPOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(DiplopodHead {
            direction: Vec2::ZERO,
        })
        .insert(DiplopodSegment)
        .insert(Position {
            x: ARENA_WIDTH / 2,
            y: ARENA_HEIGHT / 2,
        })
        .insert(crate::components::Size::square(1.0))
        .id()];
}

fn spawn_segment(commands: &mut Commands, color: Color, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite { color, ..default() },
            ..default()
        })
        .insert(DiplopodSegment)
        .insert(position)
        .insert(crate::components::Size::square(1.0))
        .id()
}

pub fn init_food(
    mut commands: Commands,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    tile_size: Res<TileSize>,
) {
    spawn_food(&mut commands, &mut free_consumable_positions, &tile_size);
}

fn spawn_food(
    commands: &mut Commands,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    tile_size: &Res<TileSize>,
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
        tile_size,
    );
}

fn spawn_random_food(
    amount: u32,
    commands: &mut Commands,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    tile_size: &Res<TileSize>,
) {
    let shape = shapes::Circle {
        radius: tile_size.0 as f32,
        center: Vec2::new(0., 0.),
    };

    for _ in 0..amount {
        match position_candidates.positions.pop() {
            None => break,
            Some(pos) => {
                commands
                    .spawn(GeometryBuilder::build_as(
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
    tile_size: Res<TileSize>,
) {
    spawn_poison(&mut commands, &mut free_consumable_positions, &tile_size);
}

fn spawn_poison(
    commands: &mut Commands,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    tile_size: &Res<TileSize>,
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
        tile_size,
    );
}

fn spawn_random_poison(
    amount: u32,
    commands: &mut Commands,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    tile_size: &Res<TileSize>,
) {
    let shape = shapes::Circle {
        radius: tile_size.0 as f32,
        center: Vec2::new(0., 0.),
    };

    for _ in 0..amount {
        match position_candidates.positions.pop() {
            None => break,
            Some(pos) => {
                commands
                    .spawn(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(POISON_FILL_COLOR),
                            outline_mode: StrokeMode::new(POISON_OUTLINE_COLOR, 7.0),
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
    tile_size: &Res<TileSize>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
        path_builder.line_to(tile_size.0 as f32 * Vec2::X);
        path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
        path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
        let cross = path_builder.build();

        commands
            .spawn(GeometryBuilder::build_as(
                &cross,
                DrawMode::Stroke(StrokeMode::new(SUPERFOOD_COLOR, 7.5)),
                Transform::default(),
            ))
            .insert(Superfood)
            .insert(pos);
        free_consumable_positions.remove(&pos);
    }
}

fn spawn_random_antidote(
    commands: &mut Commands,
    position_candidates: &mut FreeConsumablePositions,
    free_consumable_positions: &mut ResMut<FreeConsumablePositions>,
    tile_size: &Res<TileSize>,
) {
    if let Some(pos) = position_candidates.positions.pop() {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
        path_builder.line_to(tile_size.0 as f32 * Vec2::X);
        path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
        path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
        let cross = path_builder.build();

        commands
            .spawn(GeometryBuilder::build_as(
                &cross,
                DrawMode::Stroke(StrokeMode::new(ANTIDOTE_COLOR, tile_size.0 as f32 * 0.9)),
                Transform::default(),
            ))
            .insert(Antidote)
            .insert(pos);
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
    tile_size: Res<TileSize>,
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
                &tile_size,
            );

            spawn_random_poison(
                1,
                &mut commands,
                &mut position_candidates,
                &mut free_consumable_positions,
                &tile_size,
            );
        }

        let new_size = segments.0.len() as u32 + spawn_event.new_segments as u32;
        if new_size - last_special_spawn.0 > SPECIAL_SPAWN_INTERVAL {
            last_special_spawn.0 = (new_size / SPECIAL_SPAWN_INTERVAL) * SPECIAL_SPAWN_INTERVAL;

            for ent in superfood.iter() {
                let position = consumable_positions.get(ent).unwrap();
                free_consumable_positions.positions.push(*position);
                commands.entity(ent).despawn();
            }
            free_consumable_positions.shuffle();

            if last_special_spawn.0 % (SPECIAL_SPAWN_INTERVAL * 2) == 0 {
                for ent in antidotes.iter() {
                    let position = consumable_positions.get(ent).unwrap();
                    free_consumable_positions.positions.push(*position);
                    commands.entity(ent).despawn();
                }
                free_consumable_positions.shuffle();

                spawn_random_antidote(
                    &mut commands,
                    &mut position_candidates,
                    &mut free_consumable_positions,
                    &tile_size,
                );
            }

            spawn_random_superfood(
                &mut commands,
                &mut position_candidates,
                &mut free_consumable_positions,
                &tile_size,
            );
        }
    }
}

pub fn movement(
    mut heads: Query<(Entity, &DiplopodHead)>,
    mut positions: Query<&mut Position>,
    segments: ResMut<DiplopodSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOver>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head_pos.x += head.direction.x as i32;
        head_pos.y += head.direction.y as i32;

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x >= ARENA_WIDTH
            || head_pos.y >= ARENA_HEIGHT
        {
            game_over_writer.send(GameOver);
        }

        if segment_positions.contains(&head_pos)
            && (head.direction.x != 0.0 || head.direction.y != 0.0)
        {
            game_over_writer.send(GameOver);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });

        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

pub fn eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<Growth>,
    mut spawn_consumables_writer: EventWriter<SpawnConsumables>,
    mut game_over_writer: EventWriter<GameOver>,
    mut show_message_writer: EventWriter<ShowMessage>,
    food_positions: Query<(Entity, &ConsumablePosition), With<Food>>,
    superfood_positions: Query<(Entity, &ConsumablePosition), With<Superfood>>,
    poison_positions: Query<(Entity, &ConsumablePosition), With<Poison>>,
    antidote_positions: Query<(Entity, &ConsumablePosition), With<Antidote>>,
    head_positions: Query<&Position, With<DiplopodHead>>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    mut immunity_time: ResMut<ImmunityTime>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if *food_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*food_pos);
                free_consumable_positions.shuffle();
                growth_writer.send(Growth(1));

                spawn_consumables_writer.send(SpawnConsumables {
                    regular: true,
                    new_segments: 1,
                });
            }
        }

        for (ent, superfood_pos) in superfood_positions.iter() {
            if *superfood_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*superfood_pos);
                free_consumable_positions.shuffle();
                let new_segments = thread_rng().gen_range(2..10);
                growth_writer.send(Growth(new_segments));

                show_message_writer.send(ShowMessage {
                    text: new_segments.to_string(),
                    position: *superfood_pos,
                });

                spawn_consumables_writer.send(SpawnConsumables {
                    regular: false,
                    new_segments,
                });
            }
        }

        for (ent, antidote_pos) in antidote_positions.iter() {
            if *antidote_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*antidote_pos);
                immunity_time.0 += 10;
            }
        }

        for (ent, poison_pos) in poison_positions.iter() {
            if *poison_pos == head_pos.to_consumable_position() {
                if immunity_time.0 > 0 {
                    commands.entity(ent).despawn();
                    free_consumable_positions.positions.push(*poison_pos);
                    free_consumable_positions.shuffle();
                    growth_writer.send(Growth(1));
                } else {
                    game_over_writer.send(GameOver);
                }
            }
        }
    }
}

pub fn growth(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<DiplopodSegments>,
    mut growth_reader: EventReader<Growth>,
    immunity_time: Res<ImmunityTime>,
    audio: Res<Audio>,
    audio_eat: Res<AudioEat>,
) {
    if let Some(growth) = growth_reader.iter().next() {
        audio.play(audio_eat.0.clone());

        for _ in 0..growth.0 {
            segments.0.push(spawn_segment(
                &mut commands,
                if immunity_time.0 > 0 {
                    ANTIDOTE_COLOR
                } else {
                    DIPLOPOD_COLOR
                },
                last_tail_position.0.unwrap(),
            ));
        }
    }
}

pub fn move_antidote(
    mut antidotes: Query<&mut ConsumablePosition, With<Antidote>>,
    mut segment_positions: Query<&mut Position, With<DiplopodSegment>>,
) {
    for mut pos in antidotes.iter_mut() {
        let mut new_pos = *pos;
        match thread_rng().gen_range(0..4) {
            0 => new_pos.x -= 1,
            1 => new_pos.x += 1,
            2 => new_pos.y -= 1,
            3 => new_pos.y += 1,
            _ => (),
        }

        if new_pos.x < 0
            || new_pos.x >= CONSUMABLE_WIDTH
            || new_pos.y < 0
            || new_pos.y >= CONSUMABLE_HEIGHT
            || segment_positions
                .iter_mut()
                .map(|p| p.to_consumable_position())
                .any(|x| x == new_pos)
        {
            continue;
        }

        pos.x = new_pos.x;
        pos.y = new_pos.y;
    }
}

pub fn limit_immunity(mut immunity_time: ResMut<ImmunityTime>) {
    if immunity_time.0 > 0 {
        immunity_time.0 -= 1;
    }
}

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    mut segments_res: ResMut<DiplopodSegments>,
    food: Query<Entity, With<Food>>,
    superfood: Query<Entity, With<Superfood>>,
    poison: Query<Entity, With<Poison>>,
    antidotes: Query<Entity, With<Antidote>>,
    segments: Query<Entity, With<DiplopodSegment>>,
    messages: Query<Entity, With<Text>>,
    consumable_positions: Query<&ConsumablePosition>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    mut immunity_time: ResMut<ImmunityTime>,
    tile_size: Res<TileSize>,
) {
    if reader.iter().next().is_some() {
        for ent in segments.iter() {
            commands.entity(ent).despawn();
        }

        for ent in food
            .iter()
            .chain(poison.iter())
            .chain(superfood.iter())
            .chain(antidotes.iter())
            .chain(messages.iter())
        {
            let position = consumable_positions.get(ent).unwrap();
            free_consumable_positions.positions.push(*position);
            commands.entity(ent).despawn();
        }
        free_consumable_positions.shuffle();

        last_special_spawn.0 = 0;
        immunity_time.0 = 0;

        spawn_diplopod(&mut commands, &mut segments_res);
        spawn_food(&mut commands, &mut free_consumable_positions, &tile_size);
        spawn_poison(&mut commands, &mut free_consumable_positions, &tile_size);
    }
}