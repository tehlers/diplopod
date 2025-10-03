pub mod antidote;
pub mod diplopod;
pub mod fading_text;
pub mod food;
pub mod poison;
pub mod superfood;
pub mod wall;

use crate::GameState;
use crate::MAX_X;
use crate::MAX_Y;
use crate::Sounds;
use crate::despawn_screen;
use crate::highscore::Highscore;
use crate::highscore::Lastscore;
use antidote::*;
use bevy::color::palettes::css::BLUE;
use bevy::color::palettes::css::ORANGE;
use bevy::color::palettes::css::RED;
use bevy::ecs::system::SystemState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::gamepad::GamepadRumbleIntensity;
use bevy::input::gamepad::GamepadRumbleRequest;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use diplopod::*;
use fading_text::SpawnFadingText;
use food::SpawnFood;
use poison::SpawnPoison;
use rand::Rng;
use rand::rng;
use rand::seq::SliceRandom;
use std::time::Duration;
use superfood::*;
use wall::SpawnWall;

const CONSUMABLE_WIDTH: i32 = 39 + 1;
const CONSUMABLE_HEIGHT: i32 = 21 + 1;
const CONSUMABLE_SCALE_FACTOR: i32 = 2;
const ARENA_WIDTH: i32 = (CONSUMABLE_WIDTH + 1) * CONSUMABLE_SCALE_FACTOR;
const ARENA_HEIGHT: i32 = (CONSUMABLE_HEIGHT + 1) * CONSUMABLE_SCALE_FACTOR;
const TILE_SIZE: f32 = MAX_X / ARENA_WIDTH as f32;
const UPPER_LEFT: Vec2 = Vec2::new(
    (MAX_X - (ARENA_WIDTH - 1) as f32 * TILE_SIZE) / 2.,
    (MAX_Y - (ARENA_HEIGHT - 1) as f32 * TILE_SIZE) / 2.,
);
const RADIUS_FACTOR: f32 = 0.9;

const AMOUNT_OF_FOOD: u32 = 16;
const AMOUNT_OF_POISON: u32 = 17;
const SPECIAL_SPAWN_INTERVAL: u32 = 16;

pub const ANTIDOTE_COLOR: Color = Color::WHITE;
pub const DIPLOPOD_COLOR: Color = Color::Srgba(ORANGE);
pub const DIPLOPOD_IMMUNE_COLOR: Color = Color::WHITE;
pub const FOOD_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
pub const POISON_FILL_COLOR: Color = Color::BLACK;
pub const POISON_OUTLINE_COLOR: Color = Color::Srgba(RED);
pub const SUPERFOOD_COLOR: Color = Color::Srgba(BLUE);
pub const WALL_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

type CommandResources<'a> = SystemState<(
    Commands<'a, 'a>,
    ResMut<'a, Assets<Mesh>>,
    Res<'a, DiplopodColors>,
)>;

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
enum Obstacle {
    Food,
    Poison,
    Superfood,
    Antidote,
    Wall,
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Default, Resource)]
pub struct LastSpecialSpawn(pub u32);

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
struct SpawnConsumables {
    pub regular: bool,
}

#[derive(Event)]
enum Rumble {
    Eat,
    Death,
}

#[derive(Resource)]
pub struct DiplopodColors {
    pub antidote: MeshMaterial2d<ColorMaterial>,
    pub diplopod_immune: MeshMaterial2d<ColorMaterial>,
    pub diplopod_normal: MeshMaterial2d<ColorMaterial>,
    pub food: MeshMaterial2d<ColorMaterial>,
    pub poison_fill: MeshMaterial2d<ColorMaterial>,
    pub poison_outline: MeshMaterial2d<ColorMaterial>,
    pub superfood: MeshMaterial2d<ColorMaterial>,
    pub wall: MeshMaterial2d<ColorMaterial>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_game)
            .add_systems(
                Update,
                (
                    (
                        toggle_pause.run_if(
                            input_just_pressed(KeyCode::Space)
                                .or(input_just_pressed(KeyCode::KeyP)),
                        ),
                        diplopod::keyboard,
                        diplopod::gamepad,
                        diplopod::limit_immunity,
                        superfood::rotate_superfood,
                        fading_text::fade_text,
                        (
                            diplopod::change_color_during_immunity,
                            antidote::control_antidote_sound,
                        )
                            .run_if(on_timer(Duration::from_millis(75))),
                        game_over.run_if(on_event::<GameOver>),
                    )
                        .run_if(in_state(GameState::Game)),
                    rumble,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    diplopod::movement.run_if(on_timer(Duration::from_millis(75))),
                    antidote::move_antidote.run_if(on_timer(Duration::from_millis(500))),
                    check_collision,
                    spawn_consumables.run_if(on_event::<SpawnConsumables>),
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .init_resource::<DiplopodSegments>()
            .init_resource::<LastSpecialSpawn>()
            .add_event::<GameOver>()
            .add_event::<Rumble>()
            .add_event::<SpawnConsumables>();
    }
}

fn setup_game(mut commands: Commands) {
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
fn spawn_consumables(
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

            if last_special_spawn
                .0
                .is_multiple_of(SPECIAL_SPAWN_INTERVAL * 2)
            {
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

fn check_collision(
    mut commands: Commands,
    mut heads: Query<(&mut DiplopodHead, &Transform)>,
    obstacles: Query<(Entity, &Transform, &Obstacle)>,
    mut spawn_consumables_writer: EventWriter<SpawnConsumables>,
    mut game_over_writer: EventWriter<GameOver>,
    mut rumble_writer: EventWriter<Rumble>,
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

                        spawn_consumables_writer.write(SpawnConsumables { regular: true });
                        rumble_writer.write(Rumble::Eat);

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

                        spawn_consumables_writer.write(SpawnConsumables { regular: false });
                        rumble_writer.write(Rumble::Eat);

                        commands.spawn((
                            AudioPlayer(sounds.super_food.clone()),
                            PlaybackSettings::DESPAWN,
                        ));
                    }

                    Obstacle::Poison => {
                        if !head.immunity.finished() {
                            commands.entity(entity).despawn();
                            commands.queue(SpawnDiplopodSegment);

                            spawn_consumables_writer.write(SpawnConsumables { regular: false });
                            rumble_writer.write(Rumble::Eat);

                            commands.spawn((
                                AudioPlayer(sounds.eat_poison.clone()),
                                PlaybackSettings::DESPAWN,
                            ));
                        } else {
                            game_over_writer.write(GameOver);
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

                        rumble_writer.write(Rumble::Eat);
                    }

                    Obstacle::Wall => {
                        game_over_writer.write(GameOver);
                    }
                };
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    mut segments: ResMut<DiplopodSegments>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    sounds: Res<Sounds>,
    mut game_state: ResMut<NextState<GameState>>,
    mut lastscore: ResMut<Lastscore>,
    mut highscore: ResMut<Highscore>,
    mut rumble_writer: EventWriter<Rumble>,
) {
    if reader.read().next().is_some() {
        commands.spawn((
            AudioPlayer(sounds.game_over.clone()),
            PlaybackSettings::DESPAWN,
        ));
        rumble_writer.write(Rumble::Death);

        lastscore.0 = segments.0.len() as u16;

        if lastscore.0 > highscore.0 {
            highscore.0 = lastscore.0;
        }

        last_special_spawn.0 = 0;

        segments.0 = Vec::new();

        game_state.set(GameState::Highscore);
    }
}

fn rumble(
    mut rumble_reader: EventReader<Rumble>,
    mut rumble_writer: EventWriter<GamepadRumbleRequest>,
    gamepads: Query<Entity, With<Gamepad>>,
) {
    if let Some(rumble) = rumble_reader.read().next() {
        for gamepad in &gamepads {
            rumble_writer.write(GamepadRumbleRequest::Add {
                gamepad,
                intensity: match rumble {
                    Rumble::Eat => GamepadRumbleIntensity::WEAK_MAX,
                    Rumble::Death => GamepadRumbleIntensity::MAX,
                },
                duration: match rumble {
                    Rumble::Eat => Duration::from_millis(100),
                    Rumble::Death => Duration::from_secs(1),
                },
            });
        }
    }
}

fn toggle_pause(mut time: ResMut<Time<Virtual>>, sounds: Query<&AudioSink>) {
    if time.is_paused() {
        for sound in sounds.iter() {
            sound.play();
        }
        time.unpause();
    } else {
        for sound in sounds.iter() {
            sound.pause();
        }
        time.pause();
    }
}
