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
    pub const CONSUMABLE_WIDTH: i32 = 39;
    pub const CONSUMABLE_HEIGHT: i32 = 21;
    pub const CONSUMABLE_SCALE_FACTOR: i32 = 2;
    pub const ARENA_WIDTH: i32 = CONSUMABLE_WIDTH * CONSUMABLE_SCALE_FACTOR;
    pub const ARENA_HEIGHT: i32 = CONSUMABLE_HEIGHT * CONSUMABLE_SCALE_FACTOR;
    pub const AMOUNT_OF_FOOD: u32 = 16;
    pub const AMOUNT_OF_POISON: u32 = 17;
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Phase {
    Input,
    Movement,
    Eat,
    Growth,
}

fn main() {
    App::build()
        .add_startup_system(setup::setup.system())
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawner::init_diplopod.system())
        .add_startup_system_to_stage(StartupStage::PostStartup, spawner::init_food.system())
        .add_startup_system_to_stage(StartupStage::PostStartup, spawner::init_poison.system())
        .insert_resource(WindowDescriptor {
            title: "Diplopod".to_string(),
            width: 400.0,
            height: 220.0,
            ..Default::default()
        })
        .insert_resource(DiplopodSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(FreeConsumablePositions::new(
            CONSUMABLE_WIDTH as i32,
            CONSUMABLE_HEIGHT as i32,
        ))
        .add_system(
            player_input::player_input
                .system()
                .label(Phase::Input)
                .before(Phase::Movement),
        )
        .add_system(game_over::game_over.system().after(Phase::Movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.075))
                .with_system(movement::movement.system().label(Phase::Movement))
                .with_system(eat::eat.system().label(Phase::Eat).after(Phase::Movement))
                .with_system(
                    growth::growth
                        .system()
                        .label(Phase::Growth)
                        .after(Phase::Eat),
                )
                .with_system(
                    spawner::spawn_new_food
                        .system()
                        .label(Phase::Growth)
                        .after(Phase::Eat),
                )
                .with_system(
                    spawner::spawn_new_superfood
                        .system()
                        .label(Phase::Growth)
                        .after(Phase::Eat),
                )
                .with_system(
                    spawner::spawn_new_poison
                        .system()
                        .label(Phase::Growth)
                        .after(Phase::Eat),
                ),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation::position_translation.system())
                .with_system(position_translation::consumable_position_translation.system())
                .with_system(size_scaling::size_scaling.system()),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_event::<GameOver>()
        .add_event::<Growth>()
        .add_event::<SpawnFood>()
        .add_event::<SpawnSuperfood>()
        .add_event::<SpawnPoison>()
        .run();
}
