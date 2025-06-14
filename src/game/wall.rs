use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{Obstacle, OnGameScreen, Position, TILE_SIZE};

const WALL_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

pub struct SpawnWall {
    pub position: Position,
}

impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(TILE_SIZE * 2.0),
            origin: shapes::RectangleOrigin::Center,
            radii: None,
        };

        let transform: Transform = self.position.into();

        world.spawn((
            ShapeBuilder::with(&shape)
                .fill(WALL_COLOR)
                .stroke((WALL_COLOR, 1.0))
                .build(),
            transform,
            Obstacle::Wall,
            OnGameScreen,
        ));
    }
}
