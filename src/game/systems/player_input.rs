use crate::game::components::*;
use crate::game::resources::Paused;
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
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    sounds: Query<&AudioSink>,
) {
    if keyboard_input.any_just_released([KeyCode::Space, KeyCode::P]) {
        commands.init_resource::<Paused>();
        for sound in sounds.iter() {
            sound.pause();
        }
    }
}

/// Continue game and all sounds when `Space` or `p` is pressed.
pub fn unpause(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    sounds: Query<&AudioSink>,
) {
    if keyboard_input.any_just_released([KeyCode::Space, KeyCode::P]) {
        commands.remove_resource::<Paused>();
        for sound in sounds.iter() {
            sound.play();
        }
    }
}
