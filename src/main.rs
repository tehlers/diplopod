mod game;
mod highscore;
mod menu;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use game::resources::*;

const TITLE: &str = "diplopod";
const MAX_X: f32 = 1920.0;
const MAX_Y: f32 = 1200.0;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            set_default_font.run_if(resource_exists::<DefaultFontHandle>),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: MAX_X,
                min_height: MAX_Y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    let sounds = Sounds {
        eat_food: asset_server.load("audio/eat_food.ogg"),
        eat_poison: asset_server.load("audio/eat_poison.ogg"),
        special_spawn: asset_server.load("audio/special_spawn.ogg"),
        super_food: asset_server.load("audio/super_food.ogg"),
        antidote: asset_server.load("audio/antidote.ogg"),
        game_over: asset_server.load("audio/game_over.ogg"),
    };
    commands.insert_resource(sounds);

    let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");
    commands.insert_resource(DefaultFontHandle(font));

    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.visible = false;
    }
}

fn set_default_font(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    default_font_handle: Res<DefaultFontHandle>,
) {
    if let Some(font) = fonts.remove(&default_font_handle.0) {
        fonts.insert(&TextFont::default().font, font);
        commands.remove_resource::<DefaultFontHandle>();
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
