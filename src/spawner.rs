use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn spawn_diplopod(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(DiplopodHead {
            direction: Vec2::ZERO,
        });
}
