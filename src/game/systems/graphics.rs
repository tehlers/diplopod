use bevy::prelude::*;

use bevy_prototype_lyon::prelude::*;

use crate::game::OnGameScreen;
use crate::game::components::*;
use crate::game::events::ShowMessage;
use crate::prelude::*;

pub const MAX_X: f32 = 1920.0;
pub const MAX_Y: f32 = 1200.0;
pub const TILE_SIZE: f32 = MAX_X / ARENA_WIDTH as f32;
pub const UPPER_LEFT: Vec2 = Vec2::new(
    (MAX_X - (ARENA_WIDTH - 1) as f32 * TILE_SIZE) / 2.,
    (MAX_Y - (ARENA_HEIGHT - 1) as f32 * TILE_SIZE) / 2.,
);

pub fn diplopod_position_translation(mut segments: Query<(&DiplopodPosition, &mut Transform)>) {
    for (pos, mut transform) in segments.iter_mut() {
        *transform = (*pos).into();
    }
}

pub fn rotate_superfood(mut query: Query<&mut Transform, With<Superfood>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_secs();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}

pub fn change_color(
    mut query: Query<(&mut Fill, &mut Stroke), With<DiplopodSegment>>,
    heads: Query<&DiplopodHead>,
) {
    let head = heads.single();

    if head.immunity.remaining_secs() > 2.0 {
        for (mut fill, mut stroke) in query.iter_mut() {
            fill.color = DIPLOPOD_IMMUNE_COLOR;
            stroke.color = DIPLOPOD_IMMUNE_COLOR;
        }
    } else if !head.immunity.finished() {
        for (mut fill, mut stroke) in query.iter_mut() {
            if fill.color == DIPLOPOD_IMMUNE_COLOR {
                fill.color = DIPLOPOD_COLOR;
                stroke.color = DIPLOPOD_COLOR;
            } else {
                fill.color = DIPLOPOD_IMMUNE_COLOR;
                stroke.color = DIPLOPOD_IMMUNE_COLOR;
            }
        }
    } else {
        for (mut fill, mut stroke) in query.iter_mut() {
            fill.color = DIPLOPOD_COLOR;
            stroke.color = DIPLOPOD_COLOR;
        }
    }
}

pub fn fade_text(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadingText)>,
    mut writer: Text2dWriter,
) {
    for (entity, mut fading_text) in query.iter_mut() {
        writer.color(entity, 0).set_alpha(fading_text.0);
        fading_text.0 -= 0.1;

        if fading_text.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn show_message(mut commands: Commands, mut show_message_reader: EventReader<ShowMessage>) {
    if let Some(show_message) = show_message_reader.read().next() {
        commands.spawn((
            Text2d::new(&show_message.text),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor::WHITE,
            TextLayout::new_with_justify(JustifyText::Center),
            // ensure that the text is drawn above the diplopod
            Transform::from_translation(Vec3::Z),
            show_message.position,
            OnGameScreen,
            FadingText(1.0),
        ));
    }
}
