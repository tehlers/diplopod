use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy::{input::keyboard::KeyboardInput, input::ButtonState, prelude::*};

use crate::game::resources::{Highscore, Lastscore};
use crate::prelude::TITLE;

use super::{despawn_screen, Fonts, GameState};

/// Adds a screen that shows the highscore of the current session and
/// the score of the last game.
pub struct HighscorePlugin;

const TITLE_COLOR: Color = Color::ANTIQUE_WHITE;
const HEADLINE_COLOR: Color = Color::GRAY;
const HIGHSCORE_COLOR: Color = Color::WHITE;
const INITIAL_DELAY_SECONDS: u8 = 2;

#[derive(Component)]
struct OnHighscoreScreen;

#[derive(Default, Resource)]
pub struct InitialDelay(pub u8);

impl Plugin for HighscorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Highscore), setup_highscore)
            .add_systems(
                Update,
                (
                    (gamepad, keyboard).run_if(in_state(GameState::Highscore)),
                    reduce_initial_delay.run_if(on_timer(Duration::from_secs(1))),
                ),
            )
            .add_systems(
                OnExit(GameState::Highscore),
                despawn_screen::<OnHighscoreScreen>,
            )
            .insert_resource(Lastscore::default())
            .insert_resource(Highscore::default())
            .init_resource::<InitialDelay>();
    }
}

/// Reduces the initial delay of the screen that ensures that keyboard events are not processed
/// immediately after game over.
fn reduce_initial_delay(mut initial_delay: ResMut<InitialDelay>) {
    if initial_delay.0 > 0 {
        initial_delay.0 -= 1;
    }
}

/// Forwards to the menu when any key is pressed after an initial delay.
fn keyboard(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
    initial_delay: Res<InitialDelay>,
) {
    for ev in keyboard_event.iter() {
        if initial_delay.0 == 0 {
            match ev.state {
                ButtonState::Released => game_state.set(GameState::Menu),
                ButtonState::Pressed => (),
            }
        }
    }
}

/// Forwards to the menu when the A key of the gamepad is pressed after an initial delay.
pub fn gamepad(
    gamepads: Res<Gamepads>,
    buttons: Res<Input<GamepadButton>>,
    mut game_state: ResMut<NextState<GameState>>,
    initial_delay: Res<InitialDelay>,
) {
    if initial_delay.0 == 0 {
        for gamepad in gamepads.iter() {
            if buttons.just_released(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South,
            }) {
                game_state.set(GameState::Menu);
            }
        }
    }
}

/// Creates the UI of the highscore screen.
fn setup_highscore(
    mut commands: Commands,
    fonts: Res<Fonts>,
    highscore: Res<Highscore>,
    lastscore: Res<Lastscore>,
    mut initial_delay: ResMut<InitialDelay>,
) {
    let title_text_style = TextStyle {
        font: fonts.regular.clone(),
        font_size: 128.0,
        color: TITLE_COLOR,
    };

    let headline_text_style = TextStyle {
        font: fonts.regular.clone(),
        font_size: 64.0,
        color: HEADLINE_COLOR,
    };

    let highscore_text_style = TextStyle {
        font: fonts.regular.clone(),
        font_size: 64.0,
        color: HIGHSCORE_COLOR,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnHighscoreScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(TITLE, title_text_style).with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(
                        TextBundle::from_section("Highscore", headline_text_style).with_style(
                            Style {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            },
                        ),
                    );

                    parent.spawn(
                        TextBundle::from_section(
                            format!("Highscore: {}", &highscore.0),
                            highscore_text_style.clone(),
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(25.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(
                        TextBundle::from_section(
                            format!("Your last score was: {}", &lastscore.0),
                            highscore_text_style.clone(),
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(25.0)),
                            ..default()
                        }),
                    );
                });
        });

    initial_delay.0 = INITIAL_DELAY_SECONDS;
}