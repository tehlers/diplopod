use crate::components::{Food, Poison, Size};
use crate::prelude::*;
use crate::resources::ConsumableRadius;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_prototype_lyon::prelude::*;

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

pub fn resize_consumables(
    mut reader: EventReader<WindowResized>,
    mut paths: Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
    mut consumable_radius: ResMut<ConsumableRadius>,
) {
    if let Some(resized) = reader.iter().next() {
        consumable_radius.0 = calculate_consumable_radius(resized);

        let shape = shapes::Circle {
            radius: consumable_radius.0,
            center: Vec2::new(0., 0.),
        };

        for mut path in paths.iter_mut() {
            *path = ShapePath::build_as(&shape);
        }
    }
}

fn calculate_consumable_radius(resized: &WindowResized) -> f32 {
    if resized.width < resized.height {
        resized.width / ARENA_WIDTH as f32
    } else {
        resized.height / ARENA_HEIGHT as f32
    }
}
