pub mod antidote;
pub mod components;
pub mod diplopod;
pub mod events;
pub mod fading_text;
pub mod food;
pub mod poison;
pub mod resources;
pub mod superfood;
pub mod systems;
pub mod wall;

use crate::GameState;
use crate::MAX_X;
use crate::MAX_Y;
use crate::despawn_screen;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::*;
use diplopod::DiplopodSegments;
use events::*;
use resources::*;
use systems::*;

pub const CONSUMABLE_WIDTH: i32 = 39 + 1;
pub const CONSUMABLE_HEIGHT: i32 = 21 + 1;
pub const CONSUMABLE_SCALE_FACTOR: i32 = 2;
pub const ARENA_WIDTH: i32 = (CONSUMABLE_WIDTH + 1) * CONSUMABLE_SCALE_FACTOR;
pub const ARENA_HEIGHT: i32 = (CONSUMABLE_HEIGHT + 1) * CONSUMABLE_SCALE_FACTOR;
pub const TILE_SIZE: f32 = MAX_X / ARENA_WIDTH as f32;
pub const UPPER_LEFT: Vec2 = Vec2::new(
    (MAX_X - (ARENA_WIDTH - 1) as f32 * TILE_SIZE) / 2.,
    (MAX_Y - (ARENA_HEIGHT - 1) as f32 * TILE_SIZE) / 2.,
);
pub const RADIUS_FACTOR: f32 = 0.9;

pub const AMOUNT_OF_FOOD: u32 = 16;
pub const AMOUNT_OF_POISON: u32 = 17;
pub const SPECIAL_SPAWN_INTERVAL: u32 = 16;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Phase {
    Input,
    Movement,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
            .add_systems(OnEnter(GameState::Game), control::setup_game)
            .add_systems(
                Update,
                (
                    toggle_pause
                        .run_if(
                            input_just_pressed(KeyCode::Space)
                                .or(input_just_pressed(KeyCode::KeyP)),
                        )
                        .run_if(in_state(GameState::Game)),
                    (
                        diplopod::keyboard,
                        diplopod::gamepad,
                        antidote::move_antidote.run_if(on_timer(Duration::from_millis(500))),
                    )
                        .in_set(Phase::Input)
                        .run_if(in_state(GameState::Game)),
                    (superfood::rotate_superfood,)
                        .after(Phase::Movement)
                        .run_if(in_state(GameState::Game)),
                    (diplopod::limit_immunity, fading_text::fade_text)
                        .run_if(in_state(GameState::Game)),
                    control::game_over
                        .after(Phase::Movement)
                        .run_if(in_state(GameState::Game))
                        .run_if(on_event::<GameOver>),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    (
                        diplopod::movement
                            .after(Phase::Input)
                            .in_set(Phase::Movement),
                        control::eat,
                        control::spawn_consumables.run_if(on_event::<SpawnConsumables>),
                    )
                        .chain(),
                    (
                        diplopod::change_color_during_immunity,
                        antidote::control_antidote_sound,
                    ),
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .init_resource::<DiplopodSegments>()
            .init_resource::<LastSpecialSpawn>()
            .insert_resource(Time::<Fixed>::from_seconds(0.075))
            .add_event::<GameOver>()
            .add_event::<SpawnConsumables>();
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
