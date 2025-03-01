mod game;
mod highscore;
mod menu;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use game::resources::*;
use game::systems::*;

pub const TITLE: &str = "diplopod";
pub const MAX_X: f32 = 1920.0;
pub const MAX_Y: f32 = 1200.0;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    Highscore,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    resolution: (1149., 645.).into(),
                    ..default()
                }),
                ..default()
            }),
            EmbeddedAssetPlugin::default(), // does not work anymore in this game since Bevy 0.13.1
            menu::MenuPlugin,
            highscore::HighscorePlugin,
            game::GamePlugin,
        ))
        .add_systems(Startup, setup::setup)
        .add_systems(
            Update,
            setup::set_default_font.run_if(resource_exists::<DefaultFontHandle>),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
