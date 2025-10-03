use bevy::{ecs::system::SystemState, prelude::*};

use crate::game::CommandResources;

use super::{Obstacle, OnGameScreen, Position, RADIUS_FACTOR, TILE_SIZE};

pub struct SpawnFood {
    pub position: Position,
}

impl Command for SpawnFood {
    fn apply(self, world: &mut World) {
        let mut command_resources: CommandResources = SystemState::new(world);
        let (mut commands, mut meshes, colors) = command_resources.get_mut(world);

        let transform: Transform = self.position.into();

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(TILE_SIZE * RADIUS_FACTOR))),
            colors.food.clone(),
            transform,
            Obstacle::Food,
            OnGameScreen,
        ));

        command_resources.apply(world);
    }
}
