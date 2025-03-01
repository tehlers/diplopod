use bevy::prelude::*;

use bevy_prototype_lyon::prelude::*;

use crate::game::components::*;
use crate::prelude::*;

pub const MAX_X: f32 = 1920.0;
pub const MAX_Y: f32 = 1200.0;
pub const TILE_SIZE: f32 = MAX_X / ARENA_WIDTH as f32;
pub const UPPER_LEFT: Vec2 = Vec2::new(
    (MAX_X - (ARENA_WIDTH - 1) as f32 * TILE_SIZE) / 2.,
    (MAX_Y - (ARENA_HEIGHT - 1) as f32 * TILE_SIZE) / 2.,
);

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
