pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::despawn_screen;
use crate::prelude::*;
use crate::GameState;
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
                    control::init_food,
                    control::init_poison,
                ),
            )
            .add_systems(
                Update,
                (
                    (graphics::on_window_created, graphics::on_window_resized),
                    (
                        player_input::keyboard,
                        player_input::gamepad,
                        control::move_antidote.run_if(on_timer(Duration::from_millis(500))),
                    )
                        .in_set(Phase::Input)
                        .run_if(in_state(GameState::Game)),
                    (
                        graphics::position_translation,
                        graphics::consumable_position_translation,
                        graphics::size_scaling,
                        graphics::rotate_superfood,
                    )
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
                        .run_if(on_event::<GameOver>()),
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
                        control::spawn_consumables.run_if(on_event::<SpawnConsumables>()),
                        graphics::show_message,
                        control::growth.run_if(on_event::<Growth>()),
                    )
                        .chain(),
                    (graphics::change_color, control::control_antidote_sound),
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .insert_resource(TileSize::default())
            .insert_resource(UpperLeft::default())
            .insert_resource(DiplopodSegments::default())
            .insert_resource(LastTailPosition::default())
            .insert_resource(LastSpecialSpawn::default())
            .insert_resource(ImmunityTime::default())
            .insert_resource(FreeConsumablePositions::new(
                CONSUMABLE_WIDTH as i32,
                CONSUMABLE_HEIGHT as i32,
            ))
            .insert_resource(FixedTime::new_from_secs(0.075))
            .add_event::<GameOver>()
            .add_event::<Growth>()
            .add_event::<SpawnConsumables>()
            .add_event::<ShowMessage>();
    }
}
