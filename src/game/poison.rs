use bevy::{color::palettes::css::RED, prelude::*};
use bevy_prototype_lyon::prelude::*;

use super::{Obstacle, OnGameScreen, Position, RADIUS_FACTOR, TILE_SIZE};

const POISON_OUTLINE_COLOR: Color = Color::Srgba(RED);
const POISON_FILL_COLOR: Color = Color::BLACK;

pub struct SpawnPoison {
    pub position: Position,
}

impl Command for SpawnPoison {
    fn apply(self, world: &mut World) {
        let shape = shapes::Circle {
            radius: TILE_SIZE * RADIUS_FACTOR,
            center: Vec2::new(0., 0.),
        };

        let transform: Transform = self.position.into();

        world.spawn((
            ShapeBuilder::with(&shape)
                .fill(POISON_FILL_COLOR)
                .stroke((POISON_OUTLINE_COLOR, 7.))
                .build(),
            transform,
            Obstacle::Poison,
            OnGameScreen,
        ));
    }
}
