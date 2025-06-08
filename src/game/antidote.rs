use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{Rng, rng};

use super::{
    CONSUMABLE_HEIGHT, CONSUMABLE_WIDTH, Obstacle, OnGameScreen, Position, TILE_SIZE,
    diplopod::{DiplopodHead, DiplopodSegment},
};

const ANTIDOTE_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct Antidote;

#[derive(Component)]
pub struct AntidoteSound;

pub struct SpawnAntidote {
    pub position: Position,
}

impl Command for SpawnAntidote {
    fn apply(self, world: &mut World) {
        let cross = ShapePath::new()
            .move_to({ -TILE_SIZE } * Vec2::X)
            .line_to(TILE_SIZE * Vec2::X)
            .move_to({ -TILE_SIZE } * Vec2::Y)
            .line_to(TILE_SIZE * Vec2::Y)
            .close();

        let transform: Transform = self.position.into();

        world.spawn((
            ShapeBuilder::with(&cross)
                .stroke((ANTIDOTE_COLOR, TILE_SIZE * 0.9))
                .build(),
            transform,
            Obstacle::Antidote,
            Antidote,
            OnGameScreen,
        ));
    }
}
pub fn move_antidote(
    mut antidotes: Query<&mut Transform, (With<Antidote>, Without<DiplopodSegment>)>,
    mut segment_positions: Query<&mut Transform, With<DiplopodSegment>>,
) {
    for mut transform in antidotes.iter_mut() {
        let mut new_pos: Position = (*transform).into();
        match rng().random_range(0..4) {
            0 => new_pos.x -= 1,
            1 => new_pos.x += 1,
            2 => new_pos.y -= 1,
            3 => new_pos.y += 1,
            _ => (),
        }

        if new_pos.x < 1
            || new_pos.x >= CONSUMABLE_WIDTH
            || new_pos.y < 1
            || new_pos.y >= CONSUMABLE_HEIGHT
            || segment_positions
                .iter_mut()
                .map(|p| (*p).into())
                .any(|p: Position| p == new_pos)
        {
            continue;
        }

        *transform = new_pos.into();
    }
}

pub fn control_antidote_sound(
    mut commands: Commands,
    heads: Query<&DiplopodHead>,
    antidote_sound: Query<(&AudioSink, Entity), With<AntidoteSound>>,
) {
    if let Ok(head) = heads.single() {
        if head.immunity.remaining_secs() > 2.0 {
            // keep the sound and restart it, if it was already toggling
            if let Ok(sound) = antidote_sound.single() {
                if sound.0.is_paused() {
                    sound.0.play();
                }
            }
        } else if !head.immunity.finished() {
            if let Ok(sound) = antidote_sound.single() {
                sound.0.toggle_playback();
            }
        } else if let Ok(sound) = antidote_sound.single() {
            sound.0.stop();
            commands.entity(sound.1).despawn();
        }
    }
}
