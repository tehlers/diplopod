use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{Antidote, ConsumablePosition, DiplopodHead, Food, Poison, Position, Superfood},
    events::{GameOver, Growth, SpawnConsumables},
    resources::{FreeConsumablePositions, ImmunityTime},
};

pub fn eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<Growth>,
    mut spawn_consumables_writer: EventWriter<SpawnConsumables>,
    mut game_over_writer: EventWriter<GameOver>,
    food_positions: Query<(Entity, &ConsumablePosition), With<Food>>,
    superfood_positions: Query<(Entity, &ConsumablePosition), With<Superfood>>,
    poison_positions: Query<(Entity, &ConsumablePosition), With<Poison>>,
    antidote_positions: Query<(Entity, &ConsumablePosition), With<Antidote>>,
    head_positions: Query<&Position, With<DiplopodHead>>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    mut immunity_time: ResMut<ImmunityTime>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if *food_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*food_pos);
                free_consumable_positions.shuffle();
                growth_writer.send(Growth(1));

                spawn_consumables_writer.send(SpawnConsumables {
                    regular: true,
                    new_segments: 1,
                });
            }
        }

        for (ent, superfood_pos) in superfood_positions.iter() {
            if *superfood_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*superfood_pos);
                free_consumable_positions.shuffle();
                let new_segments = thread_rng().gen_range(2..10);
                growth_writer.send(Growth(new_segments));
                spawn_consumables_writer.send(SpawnConsumables {
                    regular: false,
                    new_segments: new_segments,
                });
            }
        }

        for (ent, antidote_pos) in antidote_positions.iter() {
            if *antidote_pos == head_pos.to_consumable_position() {
                commands.entity(ent).despawn();
                free_consumable_positions.positions.push(*antidote_pos);
                immunity_time.0 = 10;
            }
        }

        for (ent, poison_pos) in poison_positions.iter() {
            if *poison_pos == head_pos.to_consumable_position() {
                if immunity_time.0 > 0 {
                    commands.entity(ent).despawn();
                    free_consumable_positions.positions.push(*poison_pos);
                    free_consumable_positions.shuffle();
                    growth_writer.send(Growth(1));
                } else {
                    game_over_writer.send(GameOver);
                }
            }
        }
    }
}
