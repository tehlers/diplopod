use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    OnGameScreen,
    components::{Obstacle, Position},
    graphics::TILE_SIZE,
};

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

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: self.position.into(),
                ..default()
            },
            Fill::color(WALL_COLOR),
            Stroke::color(WALL_COLOR),
            Obstacle::Wall,
            OnGameScreen,
        ));
    }
}
