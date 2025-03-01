use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    OnGameScreen, RADIUS_FACTOR, TILE_SIZE,
    components::{Obstacle, Position},
};

const FOOD_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

pub struct SpawnFood {
    pub position: Position,
}

impl Command for SpawnFood {
    fn apply(self, world: &mut World) {
        let shape = shapes::Circle {
            radius: TILE_SIZE * RADIUS_FACTOR,
            center: Vec2::new(0., 0.),
        };

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: self.position.into(),
                ..default()
            },
            Fill::color(FOOD_COLOR),
            Stroke::color(FOOD_COLOR),
            Obstacle::Food,
            OnGameScreen,
        ));
    }
}
