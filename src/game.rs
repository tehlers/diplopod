pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::despawn_screen;
use crate::prelude::*;
use crate::GameState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::*;
use events::*;
use resources::*;
use systems::*;

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
            .add_systems(
                OnEnter(GameState::Game),
                (
                    control::init_diplopod,
                    control::init_wall,
                    control::init_food,
                    control::init_poison,
                )
                    .chain(),
            )
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
                        player_input::keyboard,
                        player_input::gamepad,
                        control::move_antidote.run_if(on_timer(Duration::from_millis(500))),
                    )
                        .in_set(Phase::Input)
                        .run_if(in_state(GameState::Game)),
                    (graphics::diplopod_position_translation,)
                        .after(Phase::Movement)
                        .run_if(in_state(GameState::Game)),
                    (graphics::rotate_superfood,)
                        .after(Phase::Movement)
                        .run_if(in_state(GameState::Game)),
                    (
                        control::limit_immunity.run_if(on_timer(Duration::from_secs(1))),
                        graphics::fade_text.run_if(on_timer(Duration::from_millis(200))),
                    )
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
                        control::movement
                            .after(Phase::Input)
                            .in_set(Phase::Movement),
                        control::eat,
                        control::spawn_consumables.run_if(on_event::<SpawnConsumables>),
                        graphics::show_message,
                        control::growth.run_if(on_event::<Growth>),
                    )
                        .chain(),
                    (graphics::change_color, control::control_antidote_sound),
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .insert_resource(DiplopodSegments::default())
            .insert_resource(LastSpecialSpawn::default())
            .insert_resource(ImmunityTime::default())
            .insert_resource(FreePositions::new(CONSUMABLE_WIDTH, CONSUMABLE_HEIGHT))
            .insert_resource(Time::<Fixed>::from_seconds(0.075))
            .add_event::<GameOver>()
            .add_event::<Growth>()
            .add_event::<SpawnConsumables>()
            .add_event::<ShowMessage>();
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
