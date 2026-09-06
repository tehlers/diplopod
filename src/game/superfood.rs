use bevy::prelude::*;

use crate::game::DiplopodColors;

use super::{Obstacle, OnGameScreen, Position, TILE_SIZE};

const STROKE_WIDTH: f32 = 7.5;

#[derive(Component)]
pub struct Superfood;

pub fn add_mesh(
    superfood: On<Add, Superfood>,
    positions: Query<&Position>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<DiplopodColors>,
) {
    if let Ok(position) = positions.get(superfood.entity) {
        let transform: Transform = (*position).into();

        commands
            .entity(superfood.entity)
            .insert((
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE * 2.0, STROKE_WIDTH))),
                colors.superfood.clone(),
                transform,
                Obstacle::Superfood,
                OnGameScreen,
            ))
            .with_child((
                Mesh2d(meshes.add(Rectangle::new(STROKE_WIDTH, TILE_SIZE * 2.0))),
                colors.superfood.clone(),
            ));
    }
}

pub fn rotate_superfood(mut query: Query<&mut Transform, With<Superfood>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_secs();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}
