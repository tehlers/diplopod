use crate::resources::AntidoteSoundController;
use crate::resources::Fonts;
use crate::resources::Sounds;
use bevy::audio::AudioSink;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    commands.spawn(Camera2dBundle::default());

    let sounds = Sounds {
        eat_food: asset_server.load("audio/eat_food.ogg"),
        eat_poison: asset_server.load("audio/eat_poison.ogg"),
        super_food: asset_server.load("audio/super_food.ogg"),
        antidote: asset_server.load("audio/antidote.ogg"),
        game_over: asset_server.load("audio/game_over.ogg"),
    };

    let handle = audio_sinks.get_handle(audio.play_with_settings(
        sounds.antidote.clone(),
        PlaybackSettings::ONCE.with_volume(0.0),
    ));
    commands.insert_resource(AntidoteSoundController(handle));

    commands.insert_resource(sounds);

    let fonts = Fonts {
        regular: asset_server.load("fonts/AllertaStencil-Regular.ttf"),
    };
    commands.insert_resource(fonts);
}
