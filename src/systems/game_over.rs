use crate::components::*;
use crate::events::GameOver;
use crate::resources::*;
use bevy::prelude::*;

use super::spawner::*;

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    mut segments_res: ResMut<DiplopodSegments>,
    food: Query<Entity, With<Food>>,
    superfood: Query<Entity, With<Superfood>>,
    poison: Query<Entity, With<Poison>>,
    antidotes: Query<Entity, With<Antidote>>,
    segments: Query<Entity, With<DiplopodSegment>>,
    consumable_positions: Query<&ConsumablePosition>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
    mut last_special_spawn: ResMut<LastSpecialSpawn>,
    mut immunity_time: ResMut<ImmunityTime>,
) {
    if reader.iter().next().is_some() {
        for ent in segments.iter() {
            commands.entity(ent).despawn();
        }

        for ent in food
            .iter()
            .chain(poison.iter())
            .chain(superfood.iter())
            .chain(antidotes.iter())
        {
            let position = consumable_positions.get(ent).unwrap();
            free_consumable_positions.positions.push(position.clone());
            commands.entity(ent).despawn();
        }
        free_consumable_positions.shuffle();

        last_special_spawn.0 = 0;
        immunity_time.0 = 0;

        spawn_diplopod(&mut commands, &mut segments_res);
        spawn_food(&mut commands, &mut free_consumable_positions);
        spawn_poison(&mut commands, &mut free_consumable_positions);
    }
}
