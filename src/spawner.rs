use crate::components::Size;
use crate::components::*;
use crate::prelude::AMOUNT_OF_FOOD;
use crate::resources::*;
use bevy::prelude::*;

pub fn init_diplopod(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<DiplopodSegments>,
) {
    spawn_diplopod(&mut commands, &materials, &mut segments);
}

pub fn spawn_diplopod(
    commands: &mut Commands,
    materials: &Res<Materials>,
    segments: &mut ResMut<DiplopodSegments>,
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
            commands,
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

pub fn spawn_food(
    mut commands: Commands,
    materials: Res<Materials>,
    mut free_consumable_positions: ResMut<FreeConsumablePositions>,
) {
    for _ in 0..AMOUNT_OF_FOOD {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .insert(Food)
            .insert(free_consumable_positions.positions.pop().unwrap())
            .insert(Size::square(2.0));
    }
}
