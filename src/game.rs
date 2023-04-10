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
        app.add_plugin(ShapePlugin)
            .add_systems(
                (
                    control::init_diplopod,
                    control::init_food,
                    control::init_poison,
                )
                    .in_schedule(OnEnter(GameState::Game)),
            )
            .add_systems((graphics::on_window_created, graphics::on_window_resized))
            .add_systems(
                (
                    player_input::keyboard,
                    player_input::gamepad,
                    control::move_antidote.run_if(on_timer(Duration::from_millis(500))),
                )
                    .in_set(Phase::Input)
                    .in_set(OnUpdate(GameState::Game)),
            )
            .add_systems(
                (
                    control::movement
                        .after(Phase::Input)
                        .in_set(Phase::Movement),
                    control::eat,
                    control::spawn_consumables,
                    graphics::show_message,
                    control::growth,
                )
                    .chain()
                    .in_set(OnUpdate(GameState::Game))
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_systems(
                (graphics::change_color, control::control_antidote_sound)
                    .in_set(OnUpdate(GameState::Game))
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_systems(
                (
                    control::limit_immunity.run_if(on_timer(Duration::from_secs(1))),
                    graphics::fade_text.run_if(on_timer(Duration::from_millis(200))),
                )
                    .in_set(OnUpdate(GameState::Game)),
            )
            .add_systems(
                (
                    graphics::position_translation,
                    graphics::consumable_position_translation,
                    graphics::size_scaling,
                    graphics::rotate_superfood,
                )
                    .after(Phase::Movement)
                    .in_set(OnUpdate(GameState::Game)),
            )
            .add_system(
                control::game_over
                    .after(Phase::Movement)
                    .in_set(OnUpdate(GameState::Game)),
            )
            .add_system(despawn_screen::<OnGameScreen>.in_schedule(OnExit(GameState::Game)))
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
            .insert_resource(AntidoteSoundController(Option::None))
            .insert_resource(FixedTime::new_from_secs(0.075))
            .add_event::<GameOver>()
            .add_event::<Growth>()
            .add_event::<SpawnConsumables>()
            .add_event::<ShowMessage>();
    }
}
