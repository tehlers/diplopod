use bevy::prelude::*;

use crate::game::DiplopodColors;

use super::{Obstacle, OnGameScreen, Position, RADIUS_FACTOR, TILE_SIZE};

#[derive(Component)]
pub struct Food;

pub fn add_mesh(
    food: On<Add, Food>,
    positions: Query<&Position>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<DiplopodColors>,
) {
    if let Ok(position) = positions.get(food.entity) {
        let transform: Transform = (*position).into();

        commands.entity(food.entity).insert((
            Mesh2d(meshes.add(Circle::new(TILE_SIZE * RADIUS_FACTOR))),
            colors.food.clone(),
            transform,
            Obstacle::Food,
            OnGameScreen,
        ));
    }
}
