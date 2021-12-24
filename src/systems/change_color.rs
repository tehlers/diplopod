use bevy::prelude::*;

use crate::{
    components::DiplopodSegment,
    resources::{ImmunityTime, Materials},
};

pub fn change_color(
    mut query: Query<&mut Handle<ColorMaterial>, With<DiplopodSegment>>,
    materials: Res<Materials>,
    immunity_time: Res<ImmunityTime>,
) {
    if immunity_time.0 > 2 {
        for mut handle in query.iter_mut() {
            *handle = materials.antidote_material.clone();
        }
    } else if immunity_time.0 > 0 {
        for mut handle in query.iter_mut() {
            if *handle == materials.antidote_material {
                *handle = materials.diplopod_material.clone();
            } else {
                *handle = materials.antidote_material.clone();
            }
        }
    } else {
        for mut handle in query.iter_mut() {
            *handle = materials.diplopod_material.clone();
        }
    }
}
