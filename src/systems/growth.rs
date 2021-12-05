use bevy::prelude::*;

use crate::{
    events::Growth,
    resources::{DiplopodSegments, LastTailPosition, Materials},
    spawner,
};

pub fn growth(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<DiplopodSegments>,
    mut growth_reader: EventReader<Growth>,
    materials: Res<Materials>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawner::spawn_segment(
            &mut commands,
            &materials.diplopod_material,
            last_tail_position.0.unwrap(),
        ));
    }
}
