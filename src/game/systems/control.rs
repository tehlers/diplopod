use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::components::*;
use crate::game::events::*;
use crate::game::resources::*;
use crate::game::OnGameScreen;
use crate::prelude::*;
use crate::GameState;

use super::graphics::diplopod_position2translation;
use super::graphics::position2translation;
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
                transform: Transform::from_translation(diplopod_position2translation(&position)),
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
                transform: Transform::from_translation(position2translation(&self.position)),
                ..default()
            },
            Fill::color(WALL_COLOR),
            Stroke::color(WALL_COLOR),
            Obstacle::Wall,
            OnGameScreen,
            self.position,
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
                transform: Transform::from_translation(position2translation(&self.position)),
                ..default()
            },
            Fill::color(FOOD_COLOR),
            Stroke::color(FOOD_COLOR),
            Obstacle::Food,
            OnGameScreen,
            self.position,
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
                transform: Transform::from_translation(position2translation(&self.position)),
                ..default()
            },
            Fill::color(POISON_FILL_COLOR),
            Stroke::new(POISON_OUTLINE_COLOR, 7.),
            Obstacle::Poison,
            OnGameScreen,
            self.position,
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
                transform: Transform::from_translation(position2translation(&self.position)),
                ..default()
            },
            Stroke::new(SUPERFOOD_COLOR, 7.5),
            Obstacle::Superfood,
            Superfood,
            OnGameScreen,
            self.position,
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
                transform: Transform::from_translation(position2translation(&self.position)),
                ..default()
            },
            Stroke::new(ANTIDOTE_COLOR, TILE_SIZE * 0.9),
            Obstacle::Antidote,
            Antidote,
            OnGameScreen,
            self.position,
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

    let mut position_candidates: Vec<Position> =
        Vec::with_capacity(((CONSUMABLE_WIDTH - 1) * (CONSUMABLE_HEIGHT - 1)) as usize);

    for x in 1..CONSUMABLE_WIDTH {
        for y in 1..CONSUMABLE_HEIGHT {
            position_candidates.push(Position { x, y });
        }
    }

    let segment_position = DiplopodPosition {
        x: ARENA_WIDTH / 2,
        y: ARENA_HEIGHT / 2,
    }
    .to_position();
    position_candidates.retain(|&p| p != segment_position);

    position_candidates.shuffle(&mut thread_rng());

    for _ in 0..AMOUNT_OF_FOOD {
        if let Some(position) = position_candidates.pop() {
            commands.queue(SpawnFood { position });
        }
    }

    for _ in 0..AMOUNT_OF_POISON {
        if let Some(position) = position_candidates.pop() {
            commands.queue(SpawnPoison { position });
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_consumables(
    mut commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_consumables_reader: EventReader<SpawnConsumables>,
    diplopod_positions: Query<&DiplopodPosition>,
    positions: Query<&Position>,
    superfood: Query<Entity, With<Superfood>>,
    antidotes: Query<Entity, With<Antidote>>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    sounds: Res<Sounds>,
) {
    if let Some(spawn_event) = spawn_consumables_reader.read().next() {
        let mut position_candidates: Vec<Position> =
            Vec::with_capacity(((CONSUMABLE_WIDTH - 1) * (CONSUMABLE_HEIGHT - 1)) as usize);

        for x in 1..CONSUMABLE_WIDTH {
            for y in 1..CONSUMABLE_HEIGHT {
                position_candidates.push(Position { x, y });
            }
        }

        for position in positions.iter() {
            position_candidates.retain(|&p| p != *position);
        }

        for segment in segments.0.iter() {
            if let Ok(diplopod_position) = diplopod_positions.get(*segment) {
                let position = diplopod_position.to_position();
                position_candidates.retain(|&p| p != position);
            }
        }

        position_candidates.shuffle(&mut thread_rng());

        if spawn_event.regular {
            if let Some(position) = position_candidates.pop() {
                commands.queue(SpawnFood { position });
            }

            if let Some(position) = position_candidates.pop() {
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

                if let Some(position) = position_candidates.pop() {
                    commands.queue(SpawnAntidote { position });
                }
            }

            if let Some(position) = position_candidates.pop() {
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
    obstacles: Query<(Entity, &Position, &Obstacle)>,
    mut spawn_consumables_writer: EventWriter<SpawnConsumables>,
    mut game_over_writer: EventWriter<GameOver>,
    mut show_message_writer: EventWriter<ShowMessage>,
    sounds: Res<Sounds>,
) {
    for (mut head, head_position) in heads.iter_mut() {
        for (entity, position, obstacle) in obstacles.iter() {
            if *position == head_position.to_position() {
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
                        let growth = thread_rng().gen_range(2..10);
                        for _ in 0..growth {
                            commands.queue(SpawnDiplopodSegment);
                        }

                        show_message_writer.send(ShowMessage {
                            text: growth.to_string(),
                            position: *head_position,
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
    mut antidotes: Query<(&mut Position, &mut Transform), With<Antidote>>,
    mut segment_positions: Query<&mut DiplopodPosition, With<DiplopodSegment>>,
) {
    for (mut pos, mut transform) in antidotes.iter_mut() {
        let mut new_pos = *pos;
        match thread_rng().gen_range(0..4) {
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
                .map(|p| p.to_position())
                .any(|x| x == new_pos)
        {
            continue;
        }

        pos.x = new_pos.x;
        pos.y = new_pos.y;

        transform.translation = position2translation(&pos);
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
        // keep the sound
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
