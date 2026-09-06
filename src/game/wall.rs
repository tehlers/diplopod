use bevy::prelude::*;

use crate::game::DiplopodColors;

use super::{Obstacle, OnGameScreen, Position, TILE_SIZE};

#[derive(Component)]
pub struct Wall;

pub fn add_mesh(
    wall: On<Add, Wall>,
    positions: Query<&Position>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<DiplopodColors>,
) {
    if let Ok(position) = positions.get(wall.entity) {
        let transform: Transform = (*position).into();

        commands.entity(wall.entity).insert((
            Mesh2d(meshes.add(Rectangle::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0))),
            colors.wall.clone(),
            transform,
            Obstacle::Wall,
            OnGameScreen,
        ));
    }
}
