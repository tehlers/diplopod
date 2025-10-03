use bevy::{ecs::system::SystemState, prelude::*};

use crate::game::CommandResources;

use super::{Obstacle, OnGameScreen, Position, TILE_SIZE};

pub struct SpawnWall {
    pub position: Position,
}

impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        let mut command_resources: CommandResources = SystemState::new(world);
        let (mut commands, mut meshes, colors) = command_resources.get_mut(world);

        let transform: Transform = self.position.into();

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0))),
            colors.wall.clone(),
            transform,
            Obstacle::Wall,
            OnGameScreen,
        ));

        command_resources.apply(world);
    }
}
