use anyhow::anyhow;
use anyhow::Result;
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy::{input::keyboard::KeyboardInput, input::ButtonState, prelude::*};
use directories::ProjectDirs;
use std::fs::File;
use std::io::Write;

use crate::game::resources::{self, Highscore, Lastscore};
use crate::prelude::TITLE;

use super::{despawn_screen, GameState};

/// Adds a screen that shows the highscore of the current session and
/// the score of the last game.
pub struct HighscorePlugin;

const TITLE_COLOR: Color = Color::Srgba(ANTIQUE_WHITE);
const HEADLINE_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const HIGHSCORE_COLOR: Color = Color::WHITE;
const INITIAL_DELAY_MILLISECONDS: u64 = 500;

const QUALIFIER: &str = "com.github";
const ORGANIZATION: &str = "tehlers";
const HIGHSCORE: &str = "highscore";

#[derive(Component)]
struct OnHighscoreScreen;

#[derive(Default, Resource)]
pub struct InitialDelay;

impl Plugin for HighscorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Highscore), setup_highscore)
            .add_systems(
                Update,
                (
                    (gamepad, keyboard)
                        .run_if(in_state(GameState::Highscore))
                        .run_if(not(resource_exists::<InitialDelay>)),
                    remove_initial_delay
                        .run_if(on_timer(Duration::from_millis(INITIAL_DELAY_MILLISECONDS)))
                        .run_if(resource_exists::<InitialDelay>),
                ),
            )
            .add_systems(
                OnExit(GameState::Highscore),
                despawn_screen::<OnHighscoreScreen>,
            )
            .add_systems(
                Update,
                save_highscore
                    .run_if(resource_changed::<Highscore>.and(not(resource_added::<Highscore>))),
            )
            .insert_resource(load_highscore())
            .init_resource::<Lastscore>();
    }
}

/// Load highscore from platform specific data directory (e.g.
/// `$HOME/.local/share/diplopod/highscore`). If the file is invalid or inaccessible the highscore will
/// be set back to zero.
fn load_highscore() -> resources::Highscore {
    match read_highscore_from_file() {
        Ok(highscore) => Highscore(highscore),
        Err(e) => {
            warn!("{}", e);
            Highscore::default()
        }
    }
}

/// Reads the highscore from the platform specific data directory and tries to parse the value as
/// `u16`.
fn read_highscore_from_file() -> Result<u16> {
    if let Some(projects_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, TITLE) {
        Ok(std::fs::read_to_string(projects_dirs.data_dir().join(HIGHSCORE))?.parse()?)
    } else {
        Err(anyhow!(
            "Unable to retrieve path to data directory. Highscore can't be read."
        ))
    }
}

/// Save highscore to platform specific data directory (e.g.
/// `$HOME/.local/share/diplopod/highscore`). Errors will be logged but otherwise ignored.
fn save_highscore(highscore: Res<Highscore>) {
    if let Err(e) = write_highscore_to_file(highscore.0) {
        warn!("{}", e)
    }
}

/// Writes the highscore to the platform specific data directory. The file and all necessary
/// directories are created by the function.
fn write_highscore_to_file(highscore: u16) -> Result<()> {
    if let Some(projects_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, TITLE) {
        let data_dir = projects_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;

        let mut file = File::create(data_dir.join(HIGHSCORE))?;
        write!(file, "{}", &highscore)?;

        Ok(())
    } else {
        Err(anyhow!(
            "Unable to retrieve path to data directory. Highscore can't be read."
        ))
    }
}

/// Removes the initial delay of the screen that ensures that keyboard and gamepad events are not processed
/// immediately after game over.
fn remove_initial_delay(mut commands: Commands) {
    commands.remove_resource::<InitialDelay>();
}

/// Forwards to the menu when any key is pressed after an initial delay.
fn keyboard(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for ev in keyboard_event.read() {
        match ev.state {
            ButtonState::Released => game_state.set(GameState::Menu),
            ButtonState::Pressed => (),
        }
    }
}

/// Forwards to the menu when the A key of the gamepad is pressed after an initial delay.
pub fn gamepad(gamepads: Query<&Gamepad>, mut game_state: ResMut<NextState<GameState>>) {
    for gamepad in gamepads.iter() {
        if gamepad.just_released(GamepadButton::South) {
            game_state.set(GameState::Menu);
        }
    }
}

/// Creates the UI of the highscore screen.
fn setup_highscore(mut commands: Commands, highscore: Res<Highscore>, lastscore: Res<Lastscore>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnHighscoreScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(TITLE),
                        TextFont {
                            font_size: 128.0,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new("Highscore"),
                        TextFont {
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(HEADLINE_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new(format!("Highscore {}", &highscore.0)),
                        TextFont {
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(HIGHSCORE_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(25.0)),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new(format!("Your last score was {}", &lastscore.0)),
                        TextFont {
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(HIGHSCORE_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(25.0)),
                            ..default()
                        },
                    ));
                });
        });

    commands.init_resource::<InitialDelay>();
}
