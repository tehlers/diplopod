use bevy::prelude::*;

use crate::components::FadingText;

pub fn fade_text(mut commands: Commands, mut query: Query<(Entity, &mut Text, &mut FadingText)>) {
    for (entity, mut text, mut fading_text) in query.iter_mut() {
        text.sections[0].style.color.set_a(fading_text.0);
        fading_text.0 -= 0.1;

        if fading_text.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
