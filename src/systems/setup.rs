use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
