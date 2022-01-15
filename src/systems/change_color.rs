use bevy::prelude::*;

use crate::{
    components::DiplopodSegment,
    prelude::{DIPLOPOD_COLOR, DIPLOPOD_IMMUNE_COLOR},
    resources::ImmunityTime,
};

pub fn change_color(
    mut query: Query<&mut Sprite, With<DiplopodSegment>>,
    immunity_time: Res<ImmunityTime>,
) {
    if immunity_time.0 > 2 {
        for mut sprite in query.iter_mut() {
            sprite.color = DIPLOPOD_IMMUNE_COLOR;
        }
    } else if immunity_time.0 > 0 {
        for mut sprite in query.iter_mut() {
            if sprite.color == DIPLOPOD_IMMUNE_COLOR {
                sprite.color = DIPLOPOD_COLOR;
            } else {
                sprite.color = DIPLOPOD_IMMUNE_COLOR;
            }
        }
    } else {
        for mut sprite in query.iter_mut() {
            sprite.color = DIPLOPOD_COLOR;
        }
    }
}
