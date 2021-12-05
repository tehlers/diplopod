use crate::components::*;
use crate::events::GameOver;
use crate::resources::*;
use crate::spawner::*;
use bevy::prelude::*;

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    materials: Res<Materials>,
    mut segments_res: ResMut<DiplopodSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<DiplopodSegment>>,
    consumable_positions: Query<&ConsumablePosition>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    if reader.iter().next().is_some() {
        for ent in segments.iter() {
            commands.entity(ent).despawn();
        }

        for ent in food.iter() {
            let position = consumable_positions.get(ent).unwrap();
            free_consumable_positions.positions.push(position.clone());
            commands.entity(ent).despawn();
        }
        free_consumable_positions.shuffle();

        spawn_diplopod(&mut commands, &materials, &mut segments_res);
        init_food(commands, materials, free_consumable_positions);
    }
}
