use bevy::{ecs::system::SystemState, prelude::*};

use crate::game::CommandResources;

use super::{Obstacle, OnGameScreen, Position, RADIUS_FACTOR, TILE_SIZE};

const FILL_RADIUS_FACTOR: f32 = 0.7;

pub struct SpawnPoison {
    pub position: Position,
}

impl Command for SpawnPoison {
    fn apply(self, world: &mut World) {
        let mut command_resources: CommandResources = SystemState::new(world);
        let (mut commands, mut meshes, colors) = command_resources.get_mut(world);

        let transform: Transform = self.position.into();

        commands
            .spawn((
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

        command_resources.apply(world);
    }
}
