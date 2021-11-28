mod components;
mod events;
mod resources;
mod spawner;
mod systems;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use events::*;
use resources::*;
use systems::*;

mod prelude {
    pub const CONSUMABLE_WIDTH: u32 = 39;
    pub const CONSUMABLE_HEIGHT: u32 = 21;
    pub const CONSUMABLE_SCALE_FACTOR: u32 = 2;
    pub const ARENA_WIDTH: u32 = CONSUMABLE_WIDTH * CONSUMABLE_SCALE_FACTOR;
    pub const ARENA_HEIGHT: u32 = CONSUMABLE_HEIGHT * CONSUMABLE_SCALE_FACTOR;
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Phase {
    Input,
    Movement,
}

fn main() {
    App::build()
        .add_startup_system(setup::setup.system())
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawner::spawn_diplopod.system())
        .insert_resource(WindowDescriptor {
            title: "Diplopod".to_string(),
            width: 400.0,
            height: 220.0,
            ..Default::default()
        })
        .insert_resource(DiplopodSegments::default())
        .add_system(
            player_input::player_input
                .system()
                .label(Phase::Input)
                .before(Phase::Movement),
        )
        .add_system(game_over::game_over.system().after(Phase::Movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(movement::movement.system().label(Phase::Movement)),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation::position_translation.system())
                .with_system(size_scaling::size_scaling.system()),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_event::<GameOver>()
        .run();
}
