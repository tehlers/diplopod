use bevy::prelude::*;
use crate::resources::Sounds;
use crate::resources::Fonts;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let sounds = Sounds {
        eat: asset_server.load("audio/eat.ogg"),
    };
    commands.insert_resource(sounds);

    let fonts = Fonts {
        regular: asset_server.load("fonts/AllertaStencil-Regular.ttf"),
    };
    commands.insert_resource(fonts);
}
