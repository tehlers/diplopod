use crate::components::DiplopodSegment;
use crate::events::GameOver;
use crate::resources::*;
use crate::spawner::*;
use bevy::prelude::*;

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    materials: Res<Materials>,
    segments_res: ResMut<DiplopodSegments>,
    segments: Query<Entity, With<DiplopodSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in segments.iter() {
            commands.entity(ent).despawn();
        }
        spawn_diplopod(commands, materials, segments_res);
    }
}
