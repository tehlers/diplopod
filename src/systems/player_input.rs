use crate::components::*;
use bevy::prelude::*;

pub fn player_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut DiplopodHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
            direction = Vec2::new(-1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
            direction = Vec2::new(1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
            direction = Vec2::new(direction.x, 1.0);
        }

        if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
            direction = Vec2::new(direction.x, -1.0);
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}
