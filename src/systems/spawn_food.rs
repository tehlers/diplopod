use bevy::prelude::*;

use crate::{
    components::{ConsumablePosition, Position},
    events::SpawnFood,
    resources::{DiplopodSegments, FreeConsumablePositions, Materials},
    spawner,
};

pub fn spawn_food(
    commands: Commands,
    segments: ResMut<DiplopodSegments>,
    mut spawn_food_reader: EventReader<SpawnFood>,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    if spawn_food_reader.iter().next().is_some() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .map(|p| p.to_consumable_position())
            .collect::<Vec<ConsumablePosition>>();

        let mut position_candidates = free_consumable_positions.clone();
        position_candidates.remove_all(&segment_positions);

        spawner::spawn_food(
            1,
            commands,
            materials,
            position_candidates,
            free_consumable_positions,
        );
    }
}
