use bevy::prelude::*;

use crate::game::DiplopodColors;

use super::{Obstacle, OnGameScreen, Position, RADIUS_FACTOR, TILE_SIZE};

const FILL_RADIUS_FACTOR: f32 = 0.7;

#[derive(Component)]
pub struct Poison;

pub fn add_mesh(
    poison: On<Add, Poison>,
    positions: Query<&Position>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<DiplopodColors>,
) {
    if let Ok(position) = positions.get(poison.entity) {
        let transform: Transform = (*position).into();

        commands
            .entity(poison.entity)
            .insert((
                Mesh2d(meshes.add(Circle::new(TILE_SIZE * RADIUS_FACTOR))),
                colors.poison_outline.clone(),
                transform,
                Obstacle::Poison,
                OnGameScreen,
            ))
            .with_child((
                Mesh2d(meshes.add(Circle::new(TILE_SIZE * RADIUS_FACTOR * FILL_RADIUS_FACTOR))),
                colors.poison_fill.clone(),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ));
    }
}
