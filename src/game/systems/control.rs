use rand::seq::SliceRandom;
use rand::{Rng, rng};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::GameState;
use crate::game::OnGameScreen;
use crate::game::components::*;
use crate::game::events::*;
use crate::game::resources::*;
use crate::prelude::*;

use super::graphics::TILE_SIZE;

struct SpawnDiplopodSegment;

impl Command for SpawnDiplopodSegment {
    fn apply(self, world: &mut World) {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(TILE_SIZE),
            origin: shapes::RectangleOrigin::Center,
            radii: None,
        };

        let segments = &world.resource::<DiplopodSegments>().0;
        let is_head = segments.is_empty();

        let position = if is_head {
            DiplopodPosition {
                x: ARENA_WIDTH / 2,
                y: ARENA_HEIGHT / 2,
            }
        } else {
            *world
                .get::<DiplopodPosition>(*segments.last().unwrap())
                .unwrap()
        };

        let immune = if is_head {
            false
        } else {
            !world
                .get::<DiplopodHead>(*segments.first().unwrap())
                .unwrap()
                .immunity
                .finished()
        };

        let color = if immune {
            ANTIDOTE_COLOR
        } else {
            DIPLOPOD_COLOR
        };

        let mut segment = world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: position.into(),
                ..default()
            },
            Fill::color(color),
            Stroke::color(color),
            DiplopodSegment,
            position,
            OnGameScreen,
        ));

        if is_head {
            segment.insert(DiplopodHead::default());
        }
    }
}

struct SpawnWall {
    position: Position,
}

impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(TILE_SIZE * 2.0),
            origin: shapes::RectangleOrigin::Center,
            radii: None,
        };

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: self.position.into(),
                ..default()
            },
            Fill::color(WALL_COLOR),
            Stroke::color(WALL_COLOR),
            Obstacle::Wall,
            OnGameScreen,
        ));
    }
}

struct SpawnFood {
    position: Position,
}

impl Command for SpawnFood {
    fn apply(self, world: &mut World) {
        let shape = shapes::Circle {
            radius: TILE_SIZE * RADIUS_FACTOR,
            center: Vec2::new(0., 0.),
        };

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: self.position.into(),
                ..default()
            },
            Fill::color(FOOD_COLOR),
            Stroke::color(FOOD_COLOR),
            Obstacle::Food,
            OnGameScreen,
        ));
    }
}

struct SpawnPoison {
    position: Position,
}

impl Command for SpawnPoison {
    fn apply(self, world: &mut World) {
        let shape = shapes::Circle {
            radius: TILE_SIZE * RADIUS_FACTOR,
            center: Vec2::new(0., 0.),
        };

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: self.position.into(),
                ..default()
            },
            Fill::color(POISON_FILL_COLOR),
            Stroke::new(POISON_OUTLINE_COLOR, 7.),
            Obstacle::Poison,
            OnGameScreen,
        ));
    }
}

struct SpawnSuperfood {
    position: Position,
}

impl Command for SpawnSuperfood {
    fn apply(self, world: &mut World) {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to({ -TILE_SIZE } * Vec2::X);
        path_builder.line_to(TILE_SIZE * Vec2::X);
        path_builder.move_to({ -TILE_SIZE } * Vec2::Y);
        path_builder.line_to(TILE_SIZE * Vec2::Y);
        let cross = path_builder.build();

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&cross),
                transform: self.position.into(),
                ..default()
            },
            Stroke::new(SUPERFOOD_COLOR, 7.5),
            Obstacle::Superfood,
            Superfood,
            OnGameScreen,
        ));
    }
}

struct SpawnAntidote {
    position: Position,
}

impl Command for SpawnAntidote {
    fn apply(self, world: &mut World) {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to({ -TILE_SIZE } * Vec2::X);
        path_builder.line_to(TILE_SIZE * Vec2::X);
        path_builder.move_to({ -TILE_SIZE } * Vec2::Y);
        path_builder.line_to(TILE_SIZE * Vec2::Y);
        let cross = path_builder.build();

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&cross),
                transform: self.position.into(),
                ..default()
            },
            Stroke::new(ANTIDOTE_COLOR, TILE_SIZE * 0.9),
            Obstacle::Antidote,
            Antidote,
            OnGameScreen,
        ));
    }
}

pub fn setup_game(mut commands: Commands) {
    commands.queue(SpawnDiplopodSegment);

    for x in 0..CONSUMABLE_WIDTH + 1 {
        let position = Position { x, y: 0 };
        commands.queue(SpawnWall { position });

        let position = Position {
            x,
            y: CONSUMABLE_HEIGHT,
        };
        commands.queue(SpawnWall { position });
    }

    for y in 1..CONSUMABLE_HEIGHT {
        let position = Position { x: 0, y };
        commands.queue(SpawnWall { position });

        let position = Position {
            x: CONSUMABLE_WIDTH,
            y,
        };
        commands.queue(SpawnWall { position });
    }

    let mut free_positions = get_randomized_free_positions(vec![
        DiplopodPosition {
            x: ARENA_WIDTH / 2,
            y: ARENA_HEIGHT / 2,
        }
        .into(),
    ]);

    for _ in 0..AMOUNT_OF_FOOD {
        if let Some(position) = free_positions.pop() {
            commands.queue(SpawnFood { position });
        }
    }

    for _ in 0..AMOUNT_OF_POISON {
        if let Some(position) = free_positions.pop() {
            commands.queue(SpawnPoison { position });
        }
    }
}

fn get_randomized_free_positions(occupied: Vec<Position>) -> Vec<Position> {
    let mut free_positions: Vec<Position> =
        Vec::with_capacity(((CONSUMABLE_WIDTH - 1) * (CONSUMABLE_HEIGHT - 1)) as usize);

    for x in 1..CONSUMABLE_WIDTH {
        for y in 1..CONSUMABLE_HEIGHT {
            free_positions.push(Position { x, y });
        }
    }

    for position in occupied {
        free_positions.retain(|&p| p != position);
    }

    free_positions.shuffle(&mut rng());

    free_positions
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_consumables(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_consumables_reader: EventReader<SpawnConsumables>,
    obstacles: Query<&Transform>,
    superfood: Query<Entity, With<Superfood>>,
    antidotes: Query<Entity, With<Antidote>>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    sounds: Res<Sounds>,
) {
    if let Some(spawn_event) = spawn_consumables_reader.read().next() {
        let mut free_positions =
            get_randomized_free_positions(obstacles.iter().map(|o| (*o).into()).collect());

        if spawn_event.regular {
            if let Some(position) = free_positions.pop() {
                commands.queue(SpawnFood { position });
            }

            if let Some(position) = free_positions.pop() {
                commands.queue(SpawnPoison { position });
            }
        }

        if segments.0.len() as u32 - last_special_spawn.0 > SPECIAL_SPAWN_INTERVAL {
            last_special_spawn.0 =
                (segments.0.len() as u32 / SPECIAL_SPAWN_INTERVAL) * SPECIAL_SPAWN_INTERVAL;

            for ent in superfood.iter() {
                commands.entity(ent).despawn();
            }

            if last_special_spawn.0 % (SPECIAL_SPAWN_INTERVAL * 2) == 0 {
                for ent in antidotes.iter() {
                    commands.entity(ent).despawn();
                }

                if let Some(position) = free_positions.pop() {
                    commands.queue(SpawnAntidote { position });
                }
            }

            if let Some(position) = free_positions.pop() {
                commands.queue(SpawnSuperfood { position });
            }

            commands.spawn((
                AudioPlayer(sounds.special_spawn.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

pub fn movement(
    mut heads: Query<(Entity, &DiplopodHead)>,
    mut positions: Query<&mut DiplopodPosition>,
    segments: ResMut<DiplopodSegments>,
    mut game_over_writer: EventWriter<GameOver>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<DiplopodPosition>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head_pos.x += head.direction.x as i32;
        head_pos.y += head.direction.y as i32;

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
    }
}

#[allow(clippy::too_many_arguments)]
pub fn eat(
    mut commands: Commands,
    mut heads: Query<(&mut DiplopodHead, &DiplopodPosition)>,
    obstacles: Query<(Entity, &Transform, &Obstacle)>,
    mut spawn_consumables_writer: EventWriter<SpawnConsumables>,
    mut game_over_writer: EventWriter<GameOver>,
    mut show_message_writer: EventWriter<ShowMessage>,
    sounds: Res<Sounds>,
) {
    for (mut head, head_diplopod_position) in heads.iter_mut() {
        let head_position: Position = (*head_diplopod_position).into();
        for (entity, transform, obstacle) in obstacles.iter() {
            if head_position == (*transform).into() {
                match obstacle {
                    Obstacle::Food => {
                        commands.entity(entity).despawn();
                        commands.queue(SpawnDiplopodSegment);

                        spawn_consumables_writer.send(SpawnConsumables { regular: true });

                        commands.spawn((
                            AudioPlayer(sounds.eat_food.clone()),
                            PlaybackSettings::DESPAWN,
                        ));
                    }

                    Obstacle::Superfood => {
                        commands.entity(entity).despawn();
                        let growth = rng().random_range(2..10);
                        for _ in 0..growth {
                            commands.queue(SpawnDiplopodSegment);
                        }

                        show_message_writer.send(ShowMessage {
                            text: growth.to_string(),
                            position: *head_diplopod_position,
                        });

                        spawn_consumables_writer.send(SpawnConsumables { regular: false });

                        commands.spawn((
                            AudioPlayer(sounds.super_food.clone()),
                            PlaybackSettings::DESPAWN,
                        ));
                    }

                    Obstacle::Poison => {
                        if !head.immunity.finished() {
                            commands.entity(entity).despawn();
                            commands.queue(SpawnDiplopodSegment);

                            spawn_consumables_writer.send(SpawnConsumables { regular: false });

                            commands.spawn((
                                AudioPlayer(sounds.eat_poison.clone()),
                                PlaybackSettings::DESPAWN,
                            ));
                        } else {
                            game_over_writer.send(GameOver);
                        }
                    }

                    Obstacle::Antidote => {
                        commands.entity(entity).despawn();

                        if head.immunity.finished() {
                            commands.spawn((
                                AudioPlayer(sounds.antidote.clone()),
                                PlaybackSettings::LOOP,
                                AntidoteSound,
                                OnGameScreen,
                            ));
                        }

                        let remaining = head.immunity.remaining_secs();
                        head.immunity = Timer::from_seconds(10.0 + remaining, TimerMode::Once);
                    }

                    Obstacle::Wall => {
                        game_over_writer.send(GameOver);
                    }
                };
            }
        }
    }
}

pub fn move_antidote(
    mut antidotes: Query<&mut Transform, With<Antidote>>,
    mut segment_positions: Query<&mut DiplopodPosition, With<DiplopodSegment>>,
) {
    for mut transform in antidotes.iter_mut() {
        let mut new_pos: Position = (*transform).into();
        match rng().random_range(0..4) {
            0 => new_pos.x -= 1,
            1 => new_pos.x += 1,
            2 => new_pos.y -= 1,
            3 => new_pos.y += 1,
            _ => (),
        }

        if new_pos.x < 1
            || new_pos.x >= CONSUMABLE_WIDTH
            || new_pos.y < 1
            || new_pos.y >= CONSUMABLE_HEIGHT
            || segment_positions
                .iter_mut()
                .map(|p| (*p).into())
                .any(|p: Position| p == new_pos)
        {
            continue;
        }

        *transform = new_pos.into();
    }
}

pub fn limit_immunity(mut heads: Query<&mut DiplopodHead>, time: Res<Time>) {
    let mut head = heads.single_mut();

    if !head.immunity.finished() {
        head.immunity.tick(time.delta());
    }
}

pub fn control_antidote_sound(
    mut commands: Commands,
    heads: Query<&DiplopodHead>,
    antidote_sound: Query<(&AudioSink, Entity), With<AntidoteSound>>,
) {
    let head = heads.single();

    if head.immunity.remaining_secs() > 2.0 {
        // keep the sound and restart it, if it was already toggling
        if let Ok(sound) = antidote_sound.get_single() {
            if sound.0.is_paused() {
                sound.0.play();
            }
        }
    } else if !head.immunity.finished() {
        if let Ok(sound) = antidote_sound.get_single() {
            sound.0.toggle();
        }
    } else if let Ok(sound) = antidote_sound.get_single() {
        sound.0.stop();
        commands.entity(sound.1).despawn();
    }
}

#[allow(clippy::too_many_arguments)]
pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    mut segments: ResMut<DiplopodSegments>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    sounds: Res<Sounds>,
    mut game_state: ResMut<NextState<GameState>>,
    mut lastscore: ResMut<Lastscore>,
    mut highscore: ResMut<Highscore>,
) {
    if reader.read().next().is_some() {
        commands.spawn((
            AudioPlayer(sounds.game_over.clone()),
            PlaybackSettings::DESPAWN,
        ));

        lastscore.0 = segments.0.len() as u16;

        if lastscore.0 > highscore.0 {
            highscore.0 = lastscore.0;
        }

        last_special_spawn.0 = 0;

        segments.0 = Vec::new();

        game_state.set(GameState::Highscore);
    }
}
