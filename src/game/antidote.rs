use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{Rng, rng};

use crate::prelude::{CONSUMABLE_HEIGHT, CONSUMABLE_WIDTH};

use super::{
    OnGameScreen,
    components::{Obstacle, Position},
    diplopod::{DiplopodHead, DiplopodSegment},
    graphics::TILE_SIZE,
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
        let mut path_builder = PathBuilder::new();
        path_builder.move_to({ -TILE_SIZE } * Vec2::X);
        path_builder.line_to(TILE_SIZE * Vec2::X);
        path_builder.move_to({ -TILE_SIZE } * Vec2::Y);
        path_builder.line_to(TILE_SIZE * Vec2::Y);
        let cross = path_builder.build();

        world.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&cross),
                transform: self.position.into(),
                ..default()
            },
            Stroke::new(ANTIDOTE_COLOR, TILE_SIZE * 0.9),
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
    let head = heads.single();

    if head.immunity.remaining_secs() > 2.0 {
        // keep the sound and restart it, if it was already toggling
        if let Ok(sound) = antidote_sound.get_single() {
            if sound.0.is_paused() {
                sound.0.play();
            }
        }
    } else if !head.immunity.finished() {
        if let Ok(sound) = antidote_sound.get_single() {
            sound.0.toggle();
        }
    } else if let Ok(sound) = antidote_sound.get_single() {
        sound.0.stop();
        commands.entity(sound.1).despawn();
    }
}
