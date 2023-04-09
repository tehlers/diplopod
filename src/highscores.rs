use bevy::{input::keyboard::KeyboardInput, input::ButtonState, prelude::*};

use crate::resources::{Highscore, Lastscore};

use super::{despawn_screen, Fonts, GameState};

pub struct HighscoresPlugin;

const TITLE_COLOR: Color = Color::ANTIQUE_WHITE;
const HEADLINE_COLOR: Color = Color::GRAY;
const HIGHSCORE_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct OnHighscoresScreen;

impl Plugin for HighscoresPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_highscores.in_schedule(OnEnter(GameState::Highscores)))
            .add_system(keyboard.in_set(OnUpdate(GameState::Highscores)))
            .add_system(
                despawn_screen::<OnHighscoresScreen>.in_schedule(OnExit(GameState::Highscores)),
            );
    }
}

fn keyboard(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for ev in keyboard_event.iter() {
        match ev.state {
            ButtonState::Released => game_state.set(GameState::Menu),
            ButtonState::Pressed => (),
        }
    }
}

fn setup_highscores(
    mut commands: Commands,
    fonts: Res<Fonts>,
    highscore: Res<Highscore>,
    lastscore: Res<Lastscore>,
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
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnHighscoresScreen,
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
                        TextBundle::from_section("Diplopod", title_text_style).with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(
                        TextBundle::from_section("Highscores", headline_text_style).with_style(
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
}
