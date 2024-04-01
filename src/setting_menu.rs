use super::despawn_screen;
use super::GameState;
use crate::prelude::*;
use bevy::prelude::*;
pub struct SettingPlugin;

impl Plugin for SettingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Setting), setup_setting)
            .add_systems(
                Update,
                (gamepad, keyboard).run_if(in_state(GameState::Setting)),
            )
            .add_systems(
                OnExit(GameState::Setting),
                despawn_screen::<OnSettingScreen>,
            )
            .insert_resource(Selected::default());
    }
}

#[derive(Component)]
struct OnSettingScreen;

#[derive(Component, Default, Debug, PartialEq)]
pub enum SettingMenuButton {
    #[default]
    Level0,
    Level1,
    Level2,
}

impl SettingMenuButton {
    fn previous(&self) -> Self {
        match *self {
            SettingMenuButton::Level0 => SettingMenuButton::Level2,
            SettingMenuButton::Level1 => SettingMenuButton::Level0,
            SettingMenuButton::Level2 => SettingMenuButton::Level1,
        }
    }
    fn next(&self) -> Self {
        match *self {
            SettingMenuButton::Level0 => SettingMenuButton::Level1,
            SettingMenuButton::Level1 => SettingMenuButton::Level2,
            SettingMenuButton::Level2 => SettingMenuButton::Level0,
        }
    }
}

#[derive(Default, Resource, Debug)]
pub struct Selected(pub SettingMenuButton);

fn keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
    mut game_state: ResMut<NextState<GameState>>,
    mut fixed_time: ResMut<Time<Fixed>>,
    query: Query<(&mut BackgroundColor, &SettingMenuButton)>,
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
            SettingMenuButton::Level0 => {
                fixed_time.set_timestep_seconds(GAME_LEVEL0);
                game_state.set(GameState::Menu);
            }
            SettingMenuButton::Level1 => {
                fixed_time.set_timestep_seconds(GAME_LEVEL3);
                game_state.set(GameState::Menu);
            }
            SettingMenuButton::Level2 => {
                fixed_time.set_timestep_seconds(GAME_LEVEL6);
                game_state.set(GameState::Menu);
            }
        }
    }
}

pub fn gamepad(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut selected: ResMut<Selected>,
    query: Query<(&mut BackgroundColor, &SettingMenuButton)>,
    mut fixed_time: ResMut<Time<Fixed>>,
    mut game_state: ResMut<NextState<GameState>>,
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
                SettingMenuButton::Level0 => {
                    fixed_time.set_timestep_seconds(GAME_LEVEL0);
                    game_state.set(GameState::Menu);
                }
                SettingMenuButton::Level1 => {
                    fixed_time.set_timestep_seconds(GAME_LEVEL3);
                    game_state.set(GameState::Menu);
                }
                SettingMenuButton::Level2 => {
                    fixed_time.set_timestep_seconds(GAME_LEVEL6);
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}

fn update_selected_button(
    selected: &Res<Selected>,
    mut query: Query<(&mut BackgroundColor, &SettingMenuButton)>,
) {
    for (mut background_color, action) in &mut query {
        if &selected.0 == action {
            background_color.0 = BUTTON_SELECTED_BACKGROUND_COLOR;
        } else {
            background_color.0 = BUTTON_BACKGROUND_COLOR;
        }
    }
}
fn setup_setting(mut commands: Commands, selected: Res<Selected>) {
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
            OnSettingScreen,
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
                                    &SettingMenuButton::Level0,
                                ),
                                ..default()
                            },
                            SettingMenuButton::Level0,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Level 0",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: background_color(
                                    &selected.0,
                                    &SettingMenuButton::Level1,
                                ),
                                ..default()
                            },
                            SettingMenuButton::Level1,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Level 1",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: background_color(
                                    &selected.0,
                                    &SettingMenuButton::Level2,
                                ),
                                ..default()
                            },
                            SettingMenuButton::Level2,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Level 2", button_text_style));
                        });
                });
        });

    fn background_color(
        selected: &SettingMenuButton,
        button: &SettingMenuButton,
    ) -> BackgroundColor {
        if selected == button {
            return BUTTON_SELECTED_BACKGROUND_COLOR.into();
        }

        BUTTON_BACKGROUND_COLOR.into()
    }
}
