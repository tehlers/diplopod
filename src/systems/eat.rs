use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{ConsumablePosition, DiplopodHead, Food, Poison, Position, Superfood},
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
    superfood_positions: Query<(Entity, &ConsumablePosition), With<Superfood>>,
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
                growth_writer.send(Growth(1));
                spawn_food_writer.send(SpawnFood);
                if head_pos.x % 2 == 0 {
                    spawn_superfood_writer.send(SpawnSuperfood);
                }
                spawn_poison_writer.send(SpawnPoison);
            }
        }

        for (ent, superfood_pos) in superfood_positions.iter() {
            if *superfood_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*superfood_pos);
                free_consumable_positions.shuffle();
                growth_writer.send(Growth(thread_rng().gen_range(2..10)));
            }
        }

        for (_ent, poison_pos) in poison_positions.iter() {
            if *poison_pos == head_pos.to_consumable_position() {
                game_over_writer.send(GameOver);
            }
        }
    }
}
