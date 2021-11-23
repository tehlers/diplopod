use crate::components::*;
use bevy::prelude::*;

pub fn player_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut DiplopodHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction = Vec2::new(-1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction = Vec2::new(1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction = Vec2::new(direction.x, 1.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction = Vec2::new(direction.x, -1.0);
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}
