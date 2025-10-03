use bevy::{ecs::system::SystemState, prelude::*};

use crate::game::CommandResources;

use super::{Obstacle, OnGameScreen, Position, TILE_SIZE};

const STROKE_WIDTH: f32 = 7.5;

#[derive(Component)]
pub struct Superfood;

pub struct SpawnSuperfood {
    pub position: Position,
}

impl Command for SpawnSuperfood {
    fn apply(self, world: &mut World) {
        let mut command_resources: CommandResources = SystemState::new(world);
        let (mut commands, mut meshes, colors) = command_resources.get_mut(world);

        let transform: Transform = self.position.into();
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE * 2.0, STROKE_WIDTH))),
                colors.superfood.clone(),
                transform,
                Obstacle::Superfood,
                Superfood,
                OnGameScreen,
            ))
            .with_child((
                Mesh2d(meshes.add(Rectangle::new(STROKE_WIDTH, TILE_SIZE * 2.0))),
                colors.superfood.clone(),
            ));

        command_resources.apply(world);
    }
}

pub fn rotate_superfood(mut query: Query<&mut Transform, With<Superfood>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_secs();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}
