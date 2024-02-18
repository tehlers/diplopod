use crate::game::components::*;
use crate::game::resources::Paused;
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

pub fn gamepad(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut heads: Query<&mut DiplopodHead>,
) {
    const TILT: f32 = 0.9;

    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        for gamepad in gamepads.iter() {
            if let Some(left_stick_x) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            {
                if left_stick_x <= -TILT {
                    direction = Vec2::new(-1.0, 0.0);
                }

                if left_stick_x >= TILT {
                    direction = Vec2::new(1.0, 0.0);
                }
            }

            if let Some(left_stick_y) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            {
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

/// Pause game and all sounds when `Space` or `p` is pressed.
pub fn pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    sounds: Query<&AudioSink>,
) {
    if keyboard_input.any_just_released([KeyCode::Space, KeyCode::KeyP]) {
        commands.init_resource::<Paused>();
        for sound in sounds.iter() {
            sound.pause();
        }
    }
}

/// Continue game and all sounds when `Space` or `p` is pressed.
pub fn unpause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    sounds: Query<&AudioSink>,
) {
    if keyboard_input.any_just_released([KeyCode::Space, KeyCode::KeyP]) {
        commands.remove_resource::<Paused>();
        for sound in sounds.iter() {
            sound.play();
        }
    }
}
