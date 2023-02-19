use crate::resources::Fonts;
use crate::resources::Sounds;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default());

    let sounds = Sounds {
        eat_food: asset_server.load("audio/eat_food.ogg"),
        eat_poison: asset_server.load("audio/eat_poison.ogg"),
        special_spawn: asset_server.load("audio/special_spawn.ogg"),
        super_food: asset_server.load("audio/super_food.ogg"),
        antidote: asset_server.load("audio/antidote.ogg"),
        game_over: asset_server.load("audio/game_over.ogg"),
    };
    commands.insert_resource(sounds);

    let fonts = Fonts {
        regular: asset_server.load("fonts/AllertaStencil-Regular.ttf"),
    };
    commands.insert_resource(fonts);

    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_visibility(false);
    }
}
