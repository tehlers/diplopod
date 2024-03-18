use bevy::{app::AppExit, prelude::*};

use crate::prelude::*;

use super::{despawn_screen, GameState};

use super::game::OnGameScreen;

pub struct OverPlugin;

impl Plugin for OverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Over), setup_menu)
            .add_systems(
                Update,
                (gamepad, keyboard).run_if(in_state(GameState::Over)),
            )
            .add_systems(
                OnExit(GameState::Over),
                (
                    despawn_screen::<OnOverScreen>,
                    despawn_screen::<OnGameScreen>,
                )
                    .chain(),
            )
            .insert_resource(Selected::default());
    }
}

#[derive(Component)]
struct OnOverScreen;

#[derive(Component, Default, Debug, PartialEq)]
pub enum OverMenuButton {
    #[default]
    Resume,
    Highscore,
    Back,
}

impl OverMenuButton {
    fn previous(&self) -> Self {
        match *self {
            OverMenuButton::Resume => OverMenuButton::Back,
            OverMenuButton::Highscore => OverMenuButton::Resume,
            OverMenuButton::Back => OverMenuButton::Highscore,
        }
    }

    fn next(&self) -> Self {
        match *self {
            OverMenuButton::Resume => OverMenuButton::Highscore,
            OverMenuButton::Highscore => OverMenuButton::Back,
            OverMenuButton::Back => OverMenuButton::Resume,
        }
    }
}

#[derive(Default, Resource, Debug)]
pub struct Selected(pub OverMenuButton);

fn keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<(&mut BackgroundColor, &OverMenuButton)>,
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
            OverMenuButton::Resume => game_state.set(GameState::Game),
            OverMenuButton::Highscore => game_state.set(GameState::Highscore),
            OverMenuButton::Back => game_state.set(GameState::Menu),
        }
    }
}

pub fn gamepad(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut selected: ResMut<Selected>,
    query: Query<(&mut BackgroundColor, &OverMenuButton)>,
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
                OverMenuButton::Resume => game_state.set(GameState::Game),
                OverMenuButton::Highscore => game_state.set(GameState::Highscore),
                OverMenuButton::Back => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}

fn update_selected_button(
    selected: &Res<Selected>,
    mut query: Query<(&mut BackgroundColor, &OverMenuButton)>,
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
            OnOverScreen,
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
                                    &OverMenuButton::Resume,
                                ),
                                ..default()
                            },
                            OverMenuButton::Resume,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Resume",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: background_color(
                                    &selected.0,
                                    &OverMenuButton::Highscore,
                                ),
                                ..default()
                            },
                            OverMenuButton::Highscore,
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
                                    &OverMenuButton::Back,
                                ),
                                ..default()
                            },
                            OverMenuButton::Back,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });

    fn background_color(selected: &OverMenuButton, button: &OverMenuButton) -> BackgroundColor {
        if selected == button {
            return BUTTON_SELECTED_BACKGROUND_COLOR.into();
        }

        BUTTON_BACKGROUND_COLOR.into()
    }
}
