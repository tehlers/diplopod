use crate::components::{Antidote, Food, Poison, Size, Superfood};
use crate::prelude::*;
use crate::TileSize;
use crate::UpperLeft;
use bevy::prelude::*;
use bevy::window::WindowCreated;
use bevy::window::WindowResized;
use bevy_prototype_lyon::prelude::*;
use std::cmp;

pub fn size_scaling(mut q: Query<(&Size, &mut Transform)>, tile_size: Res<TileSize>) {
    for (_, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(tile_size.0 as f32, tile_size.0 as f32, 1.0);
    }
}

pub fn on_window_created(
    mut reader: EventReader<WindowCreated>,
    windows: Res<Windows>,
    paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut DrawMode), With<Antidote>>,
    )>,
    tile_size: ResMut<TileSize>,
    upper_left: ResMut<UpperLeft>,
) {
    if reader.iter().next().is_some() {
        let window = windows.primary();
        resize_consumables(
            window.width() as i32,
            window.height() as i32,
            paths,
            tile_size,
            upper_left,
        );
    }
}

pub fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut DrawMode), With<Antidote>>,
    )>,
    tile_size: ResMut<TileSize>,
    upper_left: ResMut<UpperLeft>,
) {
    if let Some(resized) = reader.iter().next() {
        resize_consumables(
            resized.width as i32,
            resized.height as i32,
            paths,
            tile_size,
            upper_left,
        );
    }
}

fn resize_consumables(
    width: i32,
    height: i32,
    mut paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut DrawMode), With<Antidote>>,
    )>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    tile_size.0 = cmp::min(width / ARENA_WIDTH, height / ARENA_HEIGHT);
    upper_left.x = (width - ARENA_WIDTH * tile_size.0) / 2;
    upper_left.y = (height - ARENA_HEIGHT * tile_size.0) / 2;

    let shape = shapes::Circle {
        radius: tile_size.0 as f32,
        center: Vec2::new(0., 0.),
    };

    for mut path in paths.p0().iter_mut() {
        *path = ShapePath::build_as(&shape);
    }

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
    path_builder.line_to(tile_size.0 as f32 * Vec2::X);
    path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
    path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
    let cross = path_builder.build();

    for mut path in paths.p1().iter_mut() {
        *path = ShapePath::build_as(&cross);
    }

    for (mut path, mut draw_mode) in paths.p2().iter_mut() {
        *path = ShapePath::build_as(&cross);
        *draw_mode = DrawMode::Stroke(StrokeMode::new(ANTIDOTE_COLOR, tile_size.0 as f32 * 0.9));
    }
}
