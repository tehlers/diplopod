use bevy::{color::palettes::css::RED, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::prelude::RADIUS_FACTOR;

use super::{
    OnGameScreen,
    components::{Obstacle, Position},
    graphics::TILE_SIZE,
};

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

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: self.position.into(),
                ..default()
            },
            Fill::color(POISON_FILL_COLOR),
            Stroke::new(POISON_OUTLINE_COLOR, 7.),
            Obstacle::Poison,
            OnGameScreen,
        ));
    }
}
