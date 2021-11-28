use crate::{components::*, resources::DiplopodSegments};
use bevy::prelude::*;

pub fn movement(
    mut heads: Query<(Entity, &DiplopodHead)>,
    mut positions: Query<&mut Position>,
    segments: ResMut<DiplopodSegments>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head_pos.x += head.direction.x as i32;
        head_pos.y += head.direction.y as i32;

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}
