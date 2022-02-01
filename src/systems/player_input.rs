use crate::components::*;
use bevy::prelude::*;

pub fn keyboard(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut DiplopodHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A, KeyCode::H]) {
            direction = Vec2::new(-1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D, KeyCode::L]) {
            direction = Vec2::new(1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W, KeyCode::K]) {
            direction = Vec2::new(direction.x, 1.0);
        }

        if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S, KeyCode::J]) {
            direction = Vec2::new(direction.x, -1.0);
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}

pub fn gamepad(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut heads: Query<&mut DiplopodHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        for gamepad in gamepads.iter().cloned() {
            if let Some(left_stick_x) = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            {
                if left_stick_x <= -0.01 {
                    direction = Vec2::new(-1.0, 0.0);
                }

                if left_stick_x >= 0.01 {
                    direction = Vec2::new(1.0, 0.0);
                }
            }

            if let Some(left_stick_y) = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
            {
                if left_stick_y <= -0.01 {
                    direction = Vec2::new(direction.x, -1.0);
                }

                if left_stick_y >= 0.01 {
                    direction = Vec2::new(direction.x, 1.0);
                }
            }
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}
