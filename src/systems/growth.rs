use bevy::prelude::*;

use crate::{
    events::Growth,
    resources::{DiplopodSegments, ImmunityTime, LastTailPosition, Materials},
};

use super::spawner;

pub fn growth(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<DiplopodSegments>,
    mut growth_reader: EventReader<Growth>,
    materials: Res<Materials>,
    immunity_time: Res<ImmunityTime>,
) {
    if let Some(growth) = growth_reader.iter().next() {
        for _ in 0..growth.0 {
            segments.0.push(spawner::spawn_segment(
                &mut commands,
                if immunity_time.0 > 0 {
                    &materials.antidote_material
                } else {
                    &materials.diplopod_material
                },
                last_tail_position.0.unwrap(),
            ));
        }
    }
}
