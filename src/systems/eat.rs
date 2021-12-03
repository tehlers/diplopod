use bevy::prelude::*;

use crate::{
    components::{ConsumablePosition, DiplopodHead, Food, Position},
    resources::FreeConsumablePositions,
};

pub fn eat(
    mut commands: Commands,
    food_positions: Query<(Entity, &ConsumablePosition), With<Food>>,
    head_positions: Query<&Position, With<DiplopodHead>>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if *food_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*food_pos);
            }
        }
    }
}
