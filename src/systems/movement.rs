use crate::prelude::*;
use crate::{components::*, events::GameOver, resources::DiplopodSegments};
use bevy::prelude::*;

pub fn movement(
    mut heads: Query<(Entity, &DiplopodHead)>,
    mut positions: Query<&mut Position>,
    segments: ResMut<DiplopodSegments>,
    mut game_over_writer: EventWriter<GameOver>,
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

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 > ARENA_WIDTH
            || head_pos.y as u32 > ARENA_HEIGHT
        {
            game_over_writer.send(GameOver);
        }

        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOver);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}