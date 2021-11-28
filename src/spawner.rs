use crate::components::Size;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn spawn_diplopod(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<DiplopodSegments>,
) {
    segments.0 = vec![commands
        .spawn_bundle(SpriteBundle {
            material: materials.diplopod_material.clone(),
            ..Default::default()
        })
        .insert(DiplopodHead {
            direction: Vec2::ZERO,
        })
        .insert(DiplopodSegment)
        .insert(Position { x: 0, y: 0 })
        .insert(Size::square(1.0))
        .id()];

    for _ in 0..20 {
        segments.0.push(spawn_segment(
            &mut commands,
            &materials.diplopod_material,
            Position { x: 0, y: 0 },
        ));
    }
}

fn spawn_segment(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            ..Default::default()
        })
        .insert(DiplopodSegment)
        .insert(position)
        .insert(Size::square(1.0))
        .id()
}
