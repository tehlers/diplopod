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
        let mut path_builder = PathBuilder::new();
        path_builder.move_to({ -TILE_SIZE } * Vec2::X);
        path_builder.line_to(TILE_SIZE * Vec2::X);
        path_builder.move_to({ -TILE_SIZE } * Vec2::Y);
        path_builder.line_to(TILE_SIZE * Vec2::Y);
        let cross = path_builder.build();

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&cross),
                transform: self.position.into(),
                ..default()
            },
            Stroke::new(SUPERFOOD_COLOR, 7.5),
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
