use bevy::prelude::*;

use crate::{components::FadingText, events::ShowMessage};

pub fn show_message(
    mut commands: Commands,
    mut show_message_reader: EventReader<ShowMessage>,
    asset_server: Res<AssetServer>,
) {
    if let Some(show_message) = show_message_reader.iter().next() {
        let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");

        let text_style = TextStyle {
            font,
            font_size: 36.0,
            color: Color::WHITE,
        };

        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(&show_message.text, text_style.clone())
                    .with_alignment(text_alignment),
                ..default()
            })
            .insert(show_message.position.clone())
            .insert(FadingText(1.0));
    }
}
