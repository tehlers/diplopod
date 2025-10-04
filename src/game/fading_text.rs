use bevy::prelude::*;

use super::OnGameScreen;

const FADING_TIME: f32 = 1.0;

#[derive(Component)]
pub struct FadingText {
    timer: Timer,
}

pub struct SpawnFadingText {
    pub text: String,
    pub transform: Transform,
}

impl Command for SpawnFadingText {
    fn apply(self, world: &mut World) {
        world.spawn((
            Text2d::new(&self.text),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor::WHITE,
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(self.transform.translation.with_z(2.0)),
            OnGameScreen,
            FadingText {
                timer: Timer::from_seconds(FADING_TIME, TimerMode::Once),
            },
        ));
    }
}

pub fn fade_text(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadingText)>,
    mut writer: Text2dWriter,
    time: Res<Time>,
) {
    for (entity, mut fading_text) in query.iter_mut() {
        fading_text.timer.tick(time.delta());

        if fading_text.timer.is_finished() {
            commands.entity(entity).despawn();
        } else {
            writer
                .color(entity, 0)
                .set_alpha(fading_text.timer.remaining_secs() / FADING_TIME);
        }
    }
}
