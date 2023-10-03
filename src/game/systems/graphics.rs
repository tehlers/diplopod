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

pub fn size_scaling(
    mut q: Query<(&crate::game::components::Size, &mut Transform)>,
    tile_size: Res<TileSize>,
) {
    for (_, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(tile_size.0 as f32, tile_size.0 as f32, 1.0);
    }
}

pub fn on_window_created(
    mut reader: EventReader<WindowCreated>,
    windows: Query<&Window, With<PrimaryWindow>>,
    paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut Stroke), With<Antidote>>,
        Query<&mut Path, With<Wall>>,
    )>,
    tile_size: ResMut<TileSize>,
    upper_left: ResMut<UpperLeft>,
) {
    if reader.iter().next().is_some() {
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

pub fn on_window_resized(
    mut reader: EventReader<WindowResized>,
    paths: ParamSet<(
        Query<&mut Path, Or<(With<Food>, With<Poison>)>>,
        Query<&mut Path, With<Superfood>>,
        Query<(&mut Path, &mut Stroke), With<Antidote>>,
        Query<&mut Path, With<Wall>>,
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
        Query<(&mut Path, &mut Stroke), With<Antidote>>,
        Query<&mut Path, With<Wall>>,
    )>,
    mut tile_size: ResMut<TileSize>,
    mut upper_left: ResMut<UpperLeft>,
) {
    tile_size.0 = cmp::min(width / ARENA_WIDTH, height / ARENA_HEIGHT);
    upper_left.x = (width - ARENA_WIDTH * tile_size.0) / 2;
    upper_left.y = (height - ARENA_HEIGHT * tile_size.0) / 2;

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
    };

    for mut path in paths.p3().iter_mut() {
        *path = ShapePath::build_as(&shape);
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
                (pos.x * tile_size.0 + upper_left.x - window.width() as i32 / 2) as f32,
                (pos.y * tile_size.0 + upper_left.y - window.height() as i32 / 2) as f32,
                0.0,
            )
        }
    }
}

pub fn consumable_position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&ConsumablePosition, &mut Transform)>,
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
        let delta = time.delta_seconds();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}

pub fn change_color(
    mut query: Query<&mut Sprite, With<DiplopodSegment>>,
    immunity_time: Res<ImmunityTime>,
) {
    if immunity_time.0 > 2 {
        for mut sprite in query.iter_mut() {
            sprite.color = DIPLOPOD_IMMUNE_COLOR;
        }
    } else if immunity_time.0 > 0 {
        for mut sprite in query.iter_mut() {
            if sprite.color == DIPLOPOD_IMMUNE_COLOR {
                sprite.color = DIPLOPOD_COLOR;
            } else {
                sprite.color = DIPLOPOD_IMMUNE_COLOR;
            }
        }
    } else {
        for mut sprite in query.iter_mut() {
            sprite.color = DIPLOPOD_COLOR;
        }
    }
}

pub fn fade_text(mut commands: Commands, mut query: Query<(Entity, &mut Text, &mut FadingText)>) {
    for (entity, mut text, mut fading_text) in query.iter_mut() {
        text.sections[0].style.color.set_a(fading_text.0);
        fading_text.0 -= 0.1;

        if fading_text.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn show_message(mut commands: Commands, mut show_message_reader: EventReader<ShowMessage>) {
    if let Some(show_message) = show_message_reader.iter().next() {
        let text_style = TextStyle {
            font_size: 36.0,
            color: Color::WHITE,
            ..default()
        };

        commands
            .spawn(Text2dBundle {
                text: Text::from_section(&show_message.text, text_style)
                    .with_alignment(TextAlignment::Center),
                ..default()
            })
            .insert(show_message.position)
            .insert(OnGameScreen)
            .insert(FadingText(1.0));
    }
}
