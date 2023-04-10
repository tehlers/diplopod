use bevy::{app::AppExit, prelude::*};

use crate::prelude::TITLE;

use super::{despawn_screen, Fonts, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(keyboard.in_set(OnUpdate(GameState::Menu)))
            .add_system(despawn_screen::<OnMenuScreen>.in_schedule(OnExit(GameState::Menu)))
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
    keyboard_input: Res<Input<KeyCode>>,
    mut selected: ResMut<Selected>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    query: Query<(&mut BackgroundColor, &MenuButton)>,
) {
    if keyboard_input.any_just_released([KeyCode::Up, KeyCode::W, KeyCode::K]) {
        selected.0 = selected.0.previous();
        update_selected_button(&selected.into(), query);
        return;
    }

    if keyboard_input.any_just_released([KeyCode::Down, KeyCode::S, KeyCode::J]) {
        selected.0 = selected.0.next();
        update_selected_button(&selected.into(), query);
        return;
    }

    if keyboard_input.any_just_released([KeyCode::Return, KeyCode::Space]) {
        match &selected.0 {
            MenuButton::Play => game_state.set(GameState::Game),
            MenuButton::Highscore => game_state.set(GameState::Highscore),
            MenuButton::Quit => app_exit_events.send(AppExit),
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

fn setup_menu(mut commands: Commands, fonts: Res<Fonts>, selected: Res<Selected>) {
    let button_style = Style {
        size: Size::new(Val::Px(320.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: fonts.regular.clone(),
        font_size: 64.0,
        color: BUTTON_TEXT_COLOR,
    };

    let title_text_style = TextStyle {
        font: fonts.regular.clone(),
        font_size: 128.0,
        color: TITLE_COLOR,
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
                                background_color: background_color(&selected.0, &MenuButton::Play),
                                ..default()
                            },
                            MenuButton::Play,
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
                                    &MenuButton::Highscore,
                                ),
                                ..default()
                            },
                            MenuButton::Highscore,
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
                                background_color: background_color(&selected.0, &MenuButton::Quit),
                                ..default()
                            },
                            MenuButton::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
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
