use rand::seq::SliceRandom;
use rand::{Rng, rng};

use bevy::prelude::*;

use crate::GameState;
use crate::game::antidote::{Antidote, AntidoteSound, SpawnAntidote};
use crate::game::components::*;
use crate::game::diplopod::{DiplopodHead, DiplopodSegments, START_POSITION, SpawnDiplopodSegment};
use crate::game::events::*;
use crate::game::fading_text::SpawnFadingText;
use crate::game::food::SpawnFood;
use crate::game::poison::SpawnPoison;
use crate::game::resources::*;
use crate::game::superfood::{SpawnSuperfood, Superfood};
use crate::game::wall::SpawnWall;
use crate::game::{
    AMOUNT_OF_FOOD, AMOUNT_OF_POISON, CONSUMABLE_HEIGHT, CONSUMABLE_WIDTH, OnGameScreen,
    SPECIAL_SPAWN_INTERVAL,
};

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

    let mut free_positions = get_randomized_free_positions(vec![START_POSITION.into()]);

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

#[allow(clippy::too_many_arguments)]
pub fn eat(
    mut commands: Commands,
    mut heads: Query<(&mut DiplopodHead, &Transform)>,
    obstacles: Query<(Entity, &Transform, &Obstacle)>,
    mut spawn_consumables_writer: EventWriter<SpawnConsumables>,
    mut game_over_writer: EventWriter<GameOver>,
    sounds: Res<Sounds>,
) {
    for (mut head, head_transform) in heads.iter_mut() {
        let head_position: Position = (*head_transform).into();
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

                        commands.queue(SpawnFadingText {
                            text: growth.to_string(),
                            transform: *head_transform,
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
