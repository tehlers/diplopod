mod components;
mod events;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::*;
use events::*;
use prelude::*;
use resources::*;
use systems::*;

mod prelude {
    use bevy::prelude::Color;

    pub const CONSUMABLE_WIDTH: i32 = 39;
    pub const CONSUMABLE_HEIGHT: i32 = 21;
    pub const CONSUMABLE_SCALE_FACTOR: i32 = 2;
    pub const ARENA_WIDTH: i32 = CONSUMABLE_WIDTH * CONSUMABLE_SCALE_FACTOR;
    pub const ARENA_HEIGHT: i32 = CONSUMABLE_HEIGHT * CONSUMABLE_SCALE_FACTOR;
    pub const AMOUNT_OF_FOOD: u32 = 16;
    pub const AMOUNT_OF_POISON: u32 = 17;
    pub const SPECIAL_SPAWN_INTERVAL: u32 = 16;

    pub const DIPLOPOD_COLOR: Color = Color::ORANGE;
    pub const DIPLOPOD_IMMUNE_COLOR: Color = Color::WHITE;
    pub const FOOD_COLOR: Color = Color::GREEN;
    pub const SUPERFOOD_COLOR: Color = Color::BLUE;
    pub const POISON_OUTLINE_COLOR: Color = Color::RED;
    pub const POISON_FILL_COLOR: Color = Color::BLACK;
    pub const ANTIDOTE_COLOR: Color = Color::WHITE;
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Phase {
    Input,
    Movement,
    Eat,
    Spawn,
    Growth,
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_startup_system(setup::setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Diplopod".into(),
                resolution: (400., 220.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_system(game::init_diplopod.in_base_set(StartupSet::PostStartup))
        .add_startup_system(game::init_food.in_base_set(StartupSet::PostStartup))
        .add_startup_system(game::init_poison.in_base_set(StartupSet::PostStartup))
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
        .add_system(
            player_input::keyboard
                .in_set(Phase::Input)
                .before(Phase::Movement),
        )
        .add_system(
            player_input::gamepad
                .in_set(Phase::Input)
                .before(Phase::Movement),
        )
        .add_system(
            game::limit_immunity
                .in_set(Phase::Input)
                .before(Phase::Movement)
                .run_if(on_timer(Duration::from_secs(1))),
        )
        .add_system(
            game::move_antidote
                .in_set(Phase::Input)
                .before(Phase::Movement)
                .run_if(on_timer(Duration::from_millis(500))),
        )
        .add_system(game::game_over.after(Phase::Movement))
        .add_system(graphics::on_window_created)
        .add_system(graphics::on_window_resized)
        .add_systems((
            game::movement
                .in_set(Phase::Movement)
                .in_schedule(CoreSchedule::FixedUpdate),
            game::eat
                .in_set(Phase::Eat)
                .after(Phase::Movement)
                .in_schedule(CoreSchedule::FixedUpdate),
            game::spawn_consumables
                .in_set(Phase::Spawn)
                .after(Phase::Eat)
                .in_schedule(CoreSchedule::FixedUpdate),
            game::growth
                .in_set(Phase::Growth)
                .after(Phase::Spawn)
                .in_schedule(CoreSchedule::FixedUpdate),
            graphics::show_message
                .in_set(Phase::Spawn)
                .after(Phase::Eat)
                .in_schedule(CoreSchedule::FixedUpdate),
            graphics::change_color.in_schedule(CoreSchedule::FixedUpdate),
            game::control_antidote_sound.in_schedule(CoreSchedule::FixedUpdate),
        ))
        .add_system(
            graphics::fade_text
                .in_set(Phase::Growth)
                .after(Phase::Spawn)
                .run_if(on_timer(Duration::from_millis(200))),
        )
        .add_systems((
            graphics::position_translation.in_base_set(CoreSet::PostUpdate),
            graphics::consumable_position_translation.in_base_set(CoreSet::PostUpdate),
            graphics::size_scaling.in_base_set(CoreSet::PostUpdate),
            graphics::rotate_superfood.in_base_set(CoreSet::PostUpdate),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_event::<GameOver>()
        .add_event::<Growth>()
        .add_event::<SpawnConsumables>()
        .add_event::<ShowMessage>()
        .run();
}
