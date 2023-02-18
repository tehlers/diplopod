use bevy::prelude::*;
use crate::resources::AudioEat;
use crate::resources::FontRegular;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let eat = asset_server.load("audio/eat.ogg");
    commands.insert_resource(AudioEat(eat));

    let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");
    commands.insert_resource(FontRegular(font));
}
