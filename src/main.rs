mod components;
mod events;
mod resources;
mod systems;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
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
    pub const FOOD_COLOR: Color = Color::GREEN;
    pub const SUPERFOOD_COLOR: Color = Color::BLUE;
    pub const POISON_COLOR: Color = Color::RED;
    pub const ANTIDOTE_COLOR: Color = Color::WHITE;
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Phase {
    Input,
    Movement,
    Eat,
    Spawn,
    Growth,
}

fn main() {
    App::new()
        .add_startup_system(setup::setup)
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawner::init_diplopod)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawner::init_food)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawner::init_poison)
        .insert_resource(WindowDescriptor {
            title: "Diplopod".to_string(),
            width: 400.0,
            height: 220.0,
            ..Default::default()
        })
        .insert_resource(DiplopodSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(LastSpecialSpawn::default())
        .insert_resource(ImmunityTime::default())
        .insert_resource(FreeConsumablePositions::new(
            CONSUMABLE_WIDTH as i32,
            CONSUMABLE_HEIGHT as i32,
        ))
        .add_system(
            player_input::player_input
                .label(Phase::Input)
                .before(Phase::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(
                    limit_immunity::limit_immunity
                        .label(Phase::Input)
                        .before(Phase::Movement),
                ),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(
                    move_antidote::move_antidote
                        .label(Phase::Input)
                        .before(Phase::Movement),
                ),
        )
        .add_system(game_over::game_over.after(Phase::Movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.075))
                .with_system(movement::movement.label(Phase::Movement))
                .with_system(eat::eat.label(Phase::Eat).after(Phase::Movement))
                .with_system(
                    spawner::spawn_consumables
                        .label(Phase::Spawn)
                        .after(Phase::Eat),
                )
                .with_system(growth::growth.label(Phase::Growth).after(Phase::Spawn))
                .with_system(change_color::change_color),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation::position_translation)
                .with_system(position_translation::consumable_position_translation)
                .with_system(size_scaling::size_scaling),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_event::<GameOver>()
        .add_event::<Growth>()
        .add_event::<SpawnConsumables>()
        .run();
}
