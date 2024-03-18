use bevy::{a11y::accesskit::AutoComplete, app::AppExit, prelude::*};

use crate::prelude::TITLE;

use super::{despawn_screen, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                (gamepad, keyboard).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>)
            .insert_resource(Selected::default());
    }
}

const TITLE_COLOR: Color = Color::ANTIQUE_WHITE;
const BUTTON_TEXT_COLOR: Color = Color::GRAY;
const BUTTON_BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const BUTTON_SELECTED_BACKGROUND_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component, Default, Debug, PartialEq)]
pub enum MainMenuButton {
    #[default]
    Play,
    Setting,
    Highscore,
    Quit,
}

impl MainMenuButton {
    fn previous(&self) -> Self {
        match *self {
            MainMenuButton::Play => MainMenuButton::Quit,
            MainMenuButton::Setting => MainMenuButton::Play,
            MainMenuButton::Highscore => MainMenuButton::Setting,
            MainMenuButton::Quit => MainMenuButton::Highscore,
        }
    }

    fn next(&self) -> Self {
        match *self {
            MainMenuButton::Play => MainMenuButton::Setting,
            MainMenuButton::Setting => MainMenuButton::Highscore,
            MainMenuButton::Highscore => MainMenuButton::Quit,
            MainMenuButton::Quit => MainMenuButton::Play,
        }
    }
}

#[derive(Default, Resource, Debug)]
pub struct Selected(pub MainMenuButton);

fn keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    query: Query<(&mut BackgroundColor, &MainMenuButton)>,
) {
    if keyboard_input.any_just_released([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::KeyK]) {
        selected.0 = selected.0.previous();
        update_selected_button(&selected.into(), query);
        return;
    }

    if keyboard_input.any_just_released([KeyCode::ArrowDown, KeyCode::KeyS, KeyCode::KeyJ]) {
        selected.0 = selected.0.next();
        update_selected_button(&selected.into(), query);
        return;
    }

    if keyboard_input.any_just_released([KeyCode::Enter, KeyCode::Space]) {
        match &selected.0 {
            MainMenuButton::Play => game_state.set(GameState::Game),
            MainMenuButton::Setting => game_state.set(GameState::Setting),
            MainMenuButton::Highscore => game_state.set(GameState::Highscore),
            MainMenuButton::Quit => {
                app_exit_events.send(AppExit);
            }
        }
    }
}

pub fn gamepad(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut selected: ResMut<Selected>,
    query: Query<(&mut BackgroundColor, &MainMenuButton)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for gamepad in gamepads.iter() {
        if buttons.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadUp,
        }) {
            selected.0 = selected.0.previous();
            update_selected_button(&selected.into(), query);
            return;
        }

        if buttons.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadDown,
        }) {
            selected.0 = selected.0.next();
            update_selected_button(&selected.into(), query);
            return;
        }

        if buttons.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        }) {
            match &selected.0 {
                MainMenuButton::Play => game_state.set(GameState::Game),
                MainMenuButton::Setting => game_state.set(GameState::Setting),
                MainMenuButton::Highscore => game_state.set(GameState::Highscore),
                MainMenuButton::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}

fn update_selected_button(
    selected: &Res<Selected>,
    mut query: Query<(&mut BackgroundColor, &MainMenuButton)>,
) {
    for (mut background_color, action) in &mut query {
        if &selected.0 == action {
            background_color.0 = BUTTON_SELECTED_BACKGROUND_COLOR;
        } else {
            background_color.0 = BUTTON_BACKGROUND_COLOR;
        }
    }
}

fn setup_menu(mut commands: Commands, selected: Res<Selected>) {
    let button_style = Style {
        width: Val::Px(320.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 64.0,
        color: BUTTON_TEXT_COLOR,
        ..default()
    };

    let title_text_style = TextStyle {
        font_size: 128.0,
        color: TITLE_COLOR,
        ..default()
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
            OnMenuScreen,
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

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: background_color(
                                    &selected.0,
                                    &MainMenuButton::Play,
                                ),
                                ..default()
                            },
                            MainMenuButton::Play,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Play", button_text_style.clone()));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: background_color(
                                    &selected.0,
                                    &MainMenuButton::Highscore,
                                ),
                                ..default()
                            },
                            MainMenuButton::Setting,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Setting",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: background_color(
                                    &selected.0,
                                    &MainMenuButton::Highscore,
                                ),
                                ..default()
                            },
                            MainMenuButton::Highscore,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Highscore",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: background_color(
                                    &selected.0,
                                    &MainMenuButton::Quit,
                                ),
                                ..default()
                            },
                            MainMenuButton::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });

    fn background_color(selected: &MainMenuButton, button: &MainMenuButton) -> BackgroundColor {
        if selected == button {
            return BUTTON_SELECTED_BACKGROUND_COLOR.into();
        }

        BUTTON_BACKGROUND_COLOR.into()
    }
}
