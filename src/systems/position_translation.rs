use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform)>,
    tile_size: Res<TileSize>,
    upper_left: Res<UpperLeft>,
) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            (pos.x * tile_size.0 + upper_left.x - window.width() as i32 / 2) as f32,
            (pos.y * tile_size.0 + upper_left.y - window.height() as i32 / 2) as f32,
            0.0,
        )
    }
}

pub fn consumable_position_translation(
    windows: Res<Windows>,
    mut q: Query<(&ConsumablePosition, &mut Transform)>,
    tile_size: Res<TileSize>,
    upper_left: Res<UpperLeft>,
) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            (pos.x * tile_size.0 * 2 + upper_left.x - window.width() as i32 / 2 + tile_size.0 / 2)
                as f32,
            (pos.y * tile_size.0 * 2 + upper_left.y - window.height() as i32 / 2 + tile_size.0 / 2)
                as f32,
            1.0,
        )
    }
}
