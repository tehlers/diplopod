use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{Obstacle, OnGameScreen, Position, RADIUS_FACTOR, TILE_SIZE};

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

        let transform: Transform = self.position.into();

        world.spawn((
            ShapeBuilder::with(&shape)
                .fill(FOOD_COLOR)
                .stroke((FOOD_COLOR, 1.0))
                .build(),
            transform,
            Obstacle::Food,
            OnGameScreen,
        ));
    }
}
