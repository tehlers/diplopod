mod game;
mod highscore;
mod menu;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use game::resources::*;
use game::systems::*;
use prelude::TITLE;

mod prelude {
    use bevy::{
        color::palettes::css::{BLUE, ORANGE},
        prelude::Color,
    };

    pub const TITLE: &str = "diplopod";

    pub const CONSUMABLE_WIDTH: i32 = 39 + 1;
    pub const CONSUMABLE_HEIGHT: i32 = 21 + 1;
    pub const CONSUMABLE_SCALE_FACTOR: i32 = 2;
    pub const ARENA_WIDTH: i32 = (CONSUMABLE_WIDTH + 1) * CONSUMABLE_SCALE_FACTOR;
    pub const ARENA_HEIGHT: i32 = (CONSUMABLE_HEIGHT + 1) * CONSUMABLE_SCALE_FACTOR;
    pub const AMOUNT_OF_FOOD: u32 = 16;
    pub const AMOUNT_OF_POISON: u32 = 17;
    pub const SPECIAL_SPAWN_INTERVAL: u32 = 16;

    pub const DIPLOPOD_COLOR: Color = Color::Srgba(ORANGE);
    pub const DIPLOPOD_IMMUNE_COLOR: Color = Color::WHITE;
    pub const SUPERFOOD_COLOR: Color = Color::Srgba(BLUE);

    pub const RADIUS_FACTOR: f32 = 0.9;
}

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
