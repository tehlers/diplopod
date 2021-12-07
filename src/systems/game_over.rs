use crate::components::*;
use crate::events::GameOver;
use crate::resources::*;
use bevy::prelude::*;

use super::spawner::*;

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    materials: Res<Materials>,
    mut segments_res: ResMut<DiplopodSegments>,
    food: Query<Entity, With<Food>>,
    superfood: Query<Entity, With<Superfood>>,
    poison: Query<Entity, With<Poison>>,
    segments: Query<Entity, With<DiplopodSegment>>,
    consumable_positions: Query<&ConsumablePosition>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    if reader.iter().next().is_some() {
        for ent in segments.iter() {
            commands.entity(ent).despawn();
        }

        for ent in food.iter().chain(poison.iter()).chain(superfood.iter()) {
            let position = consumable_positions.get(ent).unwrap();
            free_consumable_positions.positions.push(position.clone());
            commands.entity(ent).despawn();
        }
        free_consumable_positions.shuffle();

        spawn_diplopod(&mut commands, &materials, &mut segments_res);
        spawn_food(&mut commands, &materials, &mut free_consumable_positions);
        spawn_poison(&mut commands, &materials, &mut free_consumable_positions);
    }
}
