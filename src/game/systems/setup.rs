use crate::game::resources::DefaultFontHandle;
use crate::game::resources::Sounds;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    let sounds = Sounds {
        eat_food: asset_server.load("embedded://audio/eat_food.ogg"),
        eat_poison: asset_server.load("embedded://audio/eat_poison.ogg"),
        special_spawn: asset_server.load("embedded://audio/special_spawn.ogg"),
        super_food: asset_server.load("embedded://audio/super_food.ogg"),
        antidote: asset_server.load("embedded://audio/antidote.ogg"),
        game_over: asset_server.load("embedded://audio/game_over.ogg"),
    };
    commands.insert_resource(sounds);

    let font = asset_server.load("embedded://fonts/AllertaStencil-Regular.ttf");
    commands.insert_resource(DefaultFontHandle(font));

    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = false;
    }
}

pub fn set_default_font(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    default_font_handle: Res<DefaultFontHandle>,
) {
    if let Some(font) = fonts.remove(&default_font_handle.0) {
        fonts.insert(TextStyle::default().font, font);
        commands.remove_resource::<DefaultFontHandle>();
    }
}
