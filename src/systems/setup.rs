use crate::resources::Materials;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        diplopod_material: materials.add(Color::ORANGE.into()),
        food_material: materials.add(Color::GREEN.into()),
        superfood_material: materials.add(Color::BLUE.into()),
        poison_material: materials.add(Color::RED.into()),
        antidote_material: materials.add(Color::WHITE.into()),
    });
}
