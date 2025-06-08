use bevy::{color::palettes::css::BLUE, prelude::*};
use bevy_prototype_lyon::prelude::*;

use super::{Obstacle, OnGameScreen, Position, TILE_SIZE};

const SUPERFOOD_COLOR: Color = Color::Srgba(BLUE);

#[derive(Component)]
pub struct Superfood;

pub struct SpawnSuperfood {
    pub position: Position,
}

impl Command for SpawnSuperfood {
    fn apply(self, world: &mut World) {
        let cross = ShapePath::new()
            .move_to({ -TILE_SIZE } * Vec2::X)
            .line_to(TILE_SIZE * Vec2::X)
            .move_to({ -TILE_SIZE } * Vec2::Y)
            .line_to(TILE_SIZE * Vec2::Y)
            .close();

        let transform: Transform = self.position.into();

        world.spawn((
            ShapeBuilder::with(&cross)
                .stroke((SUPERFOOD_COLOR, 7.5))
                .build(),
            transform,
            Obstacle::Superfood,
            Superfood,
            OnGameScreen,
        ));
    }
}

pub fn rotate_superfood(mut query: Query<&mut Transform, With<Superfood>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_secs();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}
