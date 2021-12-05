use bevy::prelude::*;

use crate::{
    components::{ConsumablePosition, DiplopodHead, Food, Position},
    events::{Growth, SpawnFood},
    resources::FreeConsumablePositions,
};

pub fn eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<Growth>,
    mut spawn_food_writer: EventWriter<SpawnFood>,
    food_positions: Query<(Entity, &ConsumablePosition), With<Food>>,
    head_positions: Query<&Position, With<DiplopodHead>>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if *food_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*food_pos);
                free_consumable_positions.shuffle();
                growth_writer.send(Growth);
                spawn_food_writer.send(SpawnFood);
            }
        }
    }
}
