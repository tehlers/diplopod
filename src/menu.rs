use bevy::{app::AppExit, color::palettes::css::ANTIQUE_WHITE, prelude::*};

use crate::TITLE;

use super::{GameState, despawn_screen};

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

const TITLE_COLOR: Color = Color::Srgba(ANTIQUE_WHITE);
const BUTTON_TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const BUTTON_BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const BUTTON_SELECTED_BACKGROUND_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component, Default, Debug, PartialEq)]
pub enum MenuButton {
    #[default]
    Play,
    Highscore,
    Quit,
}

impl MenuButton {
    fn previous(&self) -> Self {
        match *self {
            MenuButton::Play => MenuButton::Quit,
            MenuButton::Highscore => MenuButton::Play,
            MenuButton::Quit => MenuButton::Highscore,
        }
    }

    fn next(&self) -> Self {
        match *self {
            MenuButton::Play => MenuButton::Highscore,
            MenuButton::Highscore => MenuButton::Quit,
            MenuButton::Quit => MenuButton::Play,
        }
    }
}

#[derive(Default, Resource, Debug)]
pub struct Selected(pub MenuButton);

fn keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: MessageWriter<AppExit>,
    query: Query<(&mut BackgroundColor, &MenuButton)>,
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
            MenuButton::Play => game_state.set(GameState::Game),
            MenuButton::Highscore => game_state.set(GameState::Highscore),
            MenuButton::Quit => {
                app_exit_events.write(AppExit::Success);
            }
        }
    }
}

pub fn gamepad(
    gamepads: Query<&Gamepad>,
    mut selected: ResMut<Selected>,
    query: Query<(&mut BackgroundColor, &MenuButton)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    for gamepad in gamepads.iter() {
        if gamepad.just_released(GamepadButton::DPadUp) {
            selected.0 = selected.0.previous();
            update_selected_button(&selected.into(), query);
            return;
        }

        if gamepad.just_released(GamepadButton::DPadDown) {
            selected.0 = selected.0.next();
            update_selected_button(&selected.into(), query);
            return;
        }

        if gamepad.just_released(GamepadButton::South) {
            match &selected.0 {
                MenuButton::Play => game_state.set(GameState::Game),
                MenuButton::Highscore => game_state.set(GameState::Highscore),
                MenuButton::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}

fn update_selected_button(
    selected: &Res<Selected>,
    mut query: Query<(&mut BackgroundColor, &MenuButton)>,
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
    let button_node = Node {
        width: Val::Px(340.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        padding: UiRect::all(Val::Px(45.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMenuScreen,
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

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            background_color(&selected.0, &MenuButton::Play),
                            MenuButton::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Play"),
                                TextFont {
                                    font_size: 64.0,
                                    ..default()
                                },
                                TextColor(BUTTON_TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            background_color(&selected.0, &MenuButton::Highscore),
                            MenuButton::Highscore,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Highscore"),
                                TextFont {
                                    font_size: 64.0,
                                    ..default()
                                },
                                TextColor(BUTTON_TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            background_color(&selected.0, &MenuButton::Quit),
                            MenuButton::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                TextFont {
                                    font_size: 64.0,
                                    ..default()
                                },
                                TextColor(BUTTON_TEXT_COLOR),
                            ));
                        });
                });
        });

    fn background_color(selected: &MenuButton, button: &MenuButton) -> BackgroundColor {
        if selected == button {
            return BUTTON_SELECTED_BACKGROUND_COLOR.into();
        }

        BUTTON_BACKGROUND_COLOR.into()
    }
}
