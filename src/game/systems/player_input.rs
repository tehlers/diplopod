use crate::game::components::*;
use bevy::prelude::*;

pub fn keyboard(keyboard_input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut DiplopodHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA, KeyCode::KeyH]) {
            direction = Vec2::new(-1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD, KeyCode::KeyL]) {
            direction = Vec2::new(1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::KeyK]) {
            direction = Vec2::new(direction.x, 1.0);
        }

        if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS, KeyCode::KeyJ]) {
            direction = Vec2::new(direction.x, -1.0);
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}

pub fn gamepad(gamepads: Query<&Gamepad>, mut heads: Query<&mut DiplopodHead>) {
    const TILT: f32 = 0.9;

    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        for gamepad in gamepads.iter() {
            if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX) {
                if left_stick_x <= -TILT {
                    direction = Vec2::new(-1.0, 0.0);
                }

                if left_stick_x >= TILT {
                    direction = Vec2::new(1.0, 0.0);
                }
            }

            if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY) {
                if left_stick_y <= -TILT {
                    direction = Vec2::new(direction.x, -1.0);
                }

                if left_stick_y >= TILT {
                    direction = Vec2::new(direction.x, 1.0);
                }
            }
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}
