use bevy::prelude::*;

use crate::{
    components::{ConsumablePosition, DiplopodHead, Food, Poison, Position},
    events::{GameOver, Growth, SpawnFood, SpawnPoison, SpawnSuperfood},
    resources::FreeConsumablePositions,
};

pub fn eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<Growth>,
    mut spawn_food_writer: EventWriter<SpawnFood>,
    mut spawn_superfood_writer: EventWriter<SpawnSuperfood>,
    mut spawn_poison_writer: EventWriter<SpawnPoison>,
    mut game_over_writer: EventWriter<GameOver>,
    food_positions: Query<(Entity, &ConsumablePosition), With<Food>>,
    poison_positions: Query<(Entity, &ConsumablePosition), With<Poison>>,
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
                if head_pos.x % 2 == 0 {
                    spawn_superfood_writer.send(SpawnSuperfood);
                }
                spawn_poison_writer.send(SpawnPoison);
            }
        }

        for (_ent, poison_pos) in poison_positions.iter() {
            if *poison_pos == head_pos.to_consumable_position() {
                game_over_writer.send(GameOver);
            }
        }
    }
}
