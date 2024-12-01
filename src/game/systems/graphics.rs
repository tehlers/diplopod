use std::cmp;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowCreated;
use bevy::window::WindowResized;

use bevy_prototype_lyon::prelude::*;

use crate::game::components::*;
use crate::game::events::ShowMessage;
use crate::game::resources::ImmunityTime;
use crate::game::OnGameScreen;
use crate::prelude::*;
use crate::TileSize;
use crate::UpperLeft;

#[allow(clippy::type_complexity)]
pub fn on_window_created(
    mut reader: EventReader<WindowCreated>,
    windows: Query<&Window, With<PrimaryWindow>>,
    paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut Stroke), With<Antidote>>,
        Query<&mut Path, With<Wall>>,
        Query<&mut Path, With<DiplopodSegment>>,
    )>,
    tile_size: ResMut<TileSize>,
    upper_left: ResMut<UpperLeft>,
) {
    if reader.read().next().is_some() {
        if let Ok(window) = windows.get_single() {
            resize_consumables(
                window.width() as i32,
                window.height() as i32,
                paths,
                tile_size,
                upper_left,
            );
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut Stroke), With<Antidote>>,
        Query<&mut Path, With<Wall>>,
        Query<&mut Path, With<DiplopodSegment>>,
    )>,
    tile_size: ResMut<TileSize>,
    upper_left: ResMut<UpperLeft>,
) {
    if let Some(resized) = reader.read().next() {
        resize_consumables(
            resized.width as i32,
            resized.height as i32,
            paths,
            tile_size,
            upper_left,
        );
    }
}

#[allow(clippy::type_complexity)]
fn resize_consumables(
    width: i32,
    height: i32,
    mut paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut Stroke), With<Antidote>>,
        Query<&mut Path, With<Wall>>,
        Query<&mut Path, With<DiplopodSegment>>,
    )>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    tile_size.0 = cmp::min(width / ARENA_WIDTH, height / ARENA_HEIGHT);
    upper_left.x = (width - (ARENA_WIDTH - 1) * tile_size.0) / 2;
    upper_left.y = (height - (ARENA_HEIGHT - 1) * tile_size.0) / 2;

    // Resize food and poison

    let shape = shapes::Circle {
        radius: tile_size.0 as f32 * RADIUS_FACTOR,
        center: Vec2::new(0., 0.),
    };

    for mut path in paths.p0().iter_mut() {
        *path = ShapePath::build_as(&shape);
    }

    // Resize superfood

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(-tile_size.0 as f32 * Vec2::X);
    path_builder.line_to(tile_size.0 as f32 * Vec2::X);
    path_builder.move_to(-tile_size.0 as f32 * Vec2::Y);
    path_builder.line_to(tile_size.0 as f32 * Vec2::Y);
    let cross = path_builder.build();

    for mut path in paths.p1().iter_mut() {
        *path = ShapePath::build_as(&cross);
    }

    // Resize antidote

    for (mut path, mut stroke) in paths.p2().iter_mut() {
        *path = ShapePath::build_as(&cross);
        *stroke = Stroke::new(ANTIDOTE_COLOR, tile_size.0 as f32 * 0.9);
    }

    // Resize walls

    let shape = shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32 * 2.0),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    for mut path in paths.p3().iter_mut() {
        *path = ShapePath::build_as(&shape);
    }

    // Resize diplopod segments

    let shape = shapes::Rectangle {
        extents: Vec2::splat(tile_size.0 as f32),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    for mut path in paths.p4().iter_mut() {
        *path = ShapePath::build_as(&shape);
    }
}

pub fn diplopod_position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&DiplopodPosition, &mut Transform)>,
    tile_size: Res<TileSize>,
    upper_left: Res<UpperLeft>,
) {
    if let Ok(window) = windows.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                (pos.x * tile_size.0 + upper_left.x - window.width() as i32 / 2) as f32,
                (pos.y * tile_size.0 + upper_left.y - window.height() as i32 / 2) as f32,
                0.0,
            )
        }
    }
}

pub fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
    tile_size: Res<TileSize>,
    upper_left: Res<UpperLeft>,
) {
    if let Ok(window) = windows.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                (pos.x * tile_size.0 * CONSUMABLE_SCALE_FACTOR + upper_left.x
                    - window.width() as i32 / 2
                    + tile_size.0 / 2) as f32,
                (pos.y * tile_size.0 * CONSUMABLE_SCALE_FACTOR + upper_left.y
                    - window.height() as i32 / 2
                    + tile_size.0 / 2) as f32,
                1.0,
            )
        }
    }
}

pub fn rotate_superfood(mut query: Query<&mut Transform, With<Superfood>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_secs();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}

pub fn change_color(
    mut query: Query<(&mut Fill, &mut Stroke), With<DiplopodSegment>>,
    immunity_time: Res<ImmunityTime>,
) {
    if immunity_time.0 > 2 {
        for (mut fill, mut stroke) in query.iter_mut() {
            fill.color = DIPLOPOD_IMMUNE_COLOR;
            stroke.color = DIPLOPOD_IMMUNE_COLOR;
        }
    } else if immunity_time.0 > 0 {
        for (mut fill, mut stroke) in query.iter_mut() {
            if fill.color == DIPLOPOD_IMMUNE_COLOR {
                fill.color = DIPLOPOD_COLOR;
                stroke.color = DIPLOPOD_COLOR;
            } else {
                fill.color = DIPLOPOD_IMMUNE_COLOR;
                stroke.color = DIPLOPOD_IMMUNE_COLOR;
            }
        }
    } else {
        for (mut fill, mut stroke) in query.iter_mut() {
            fill.color = DIPLOPOD_COLOR;
            stroke.color = DIPLOPOD_COLOR;
        }
    }
}

pub fn fade_text(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadingText)>,
    mut writer: Text2dWriter,
) {
    for (entity, mut fading_text) in query.iter_mut() {
        writer.color(entity, 0).set_alpha(fading_text.0);
        fading_text.0 -= 0.1;

        if fading_text.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn show_message(mut commands: Commands, mut show_message_reader: EventReader<ShowMessage>) {
    if let Some(show_message) = show_message_reader.read().next() {
        commands
            .spawn((
                Text2d::new(&show_message.text),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor::WHITE,
                TextLayout::new_with_justify(JustifyText::Center),
                // ensure that the text is drawn above the diplopod
                Transform::from_translation(Vec3::Z),
            ))
            .insert(show_message.position)
            .insert(OnGameScreen)
            .insert(FadingText(1.0));
    }
}
