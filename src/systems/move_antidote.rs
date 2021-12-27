use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{Antidote, ConsumablePosition, DiplopodSegment, Position},
    prelude::{CONSUMABLE_HEIGHT, CONSUMABLE_WIDTH},
};

pub fn move_antidote(
    mut antidotes: Query<&mut ConsumablePosition, With<Antidote>>,
    mut segment_positions: Query<&mut Position, With<DiplopodSegment>>,
) {
    let blocked_positions = segment_positions
        .iter_mut()
        .map(|p| p.to_consumable_position())
        .collect::<Vec<ConsumablePosition>>();

    for mut pos in antidotes.iter_mut() {
        let mut new_pos = pos.clone();
        match thread_rng().gen_range(0..4) {
            0 => new_pos.x -= 1,
            1 => new_pos.x += 1,
            2 => new_pos.y -= 1,
            3 => new_pos.y += 1,
            _ => (),
        }

        if new_pos.x < 0
            || new_pos.x >= CONSUMABLE_WIDTH
            || new_pos.y < 0
            || new_pos.y >= CONSUMABLE_HEIGHT
            || blocked_positions.contains(&new_pos)
        {
            continue;
        }

        pos.x = new_pos.x;
        pos.y = new_pos.y;
    }
}
