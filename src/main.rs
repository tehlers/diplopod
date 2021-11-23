mod components;
mod resources;
mod spawner;
mod systems;

use crate::systems::*;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_startup_system(setup::setup.system())
        .add_plugins(DefaultPlugins)
        .add_startup_stage(
            "game setup",
            SystemStage::single(spawner::spawn_diplopod.system()),
        )
        .insert_resource(WindowDescriptor {
            title: "Diplopod".to_string(),
            width: 400.0,
            height: 220.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
