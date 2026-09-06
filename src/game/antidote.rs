use bevy::prelude::*;
use rand::{Rng, rng};

use crate::game::DiplopodColors;

use super::{
    CONSUMABLE_HEIGHT, CONSUMABLE_WIDTH, Obstacle, OnGameScreen, Position, TILE_SIZE,
    diplopod::{DiplopodHead, DiplopodSegment},
};

const STROKE_WIDTH: f32 = TILE_SIZE * 0.9;

#[derive(Component)]
pub struct Antidote;

#[derive(Component)]
pub struct AntidoteSound;

pub fn add_mesh(
    antidote: On<Add, Antidote>,
    positions: Query<&Position>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<DiplopodColors>,
) {
    if let Ok(position) = positions.get(antidote.entity) {
        let transform: Transform = (*position).into();

        commands
            .entity(antidote.entity)
            .insert((
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE * 2.0, STROKE_WIDTH))),
                colors.antidote.clone(),
                transform,
                Obstacle::Antidote,
                OnGameScreen,
            ))
            .with_child((
                Mesh2d(meshes.add(Rectangle::new(STROKE_WIDTH, TILE_SIZE * 2.0))),
                colors.antidote.clone(),
            ));
    }
}

pub fn move_antidote(
    mut antidotes: Query<&mut Transform, (With<Antidote>, Without<DiplopodSegment>)>,
    segment_positions: Query<&Transform, With<DiplopodSegment>>,
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
                .iter()
                .map(|p| (*p).into())
                .any(|p: Position| p == new_pos)
        {
            continue;
        }

        let new_transform: Transform = new_pos.into();
        *transform = new_transform.with_translation(new_transform.translation + Vec3::Z * 2.0);
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
            if let Ok(sound) = antidote_sound.single()
                && sound.0.is_paused()
            {
                sound.0.play();
            }
        } else if !head.immunity.is_finished() {
            if let Ok(sound) = antidote_sound.single() {
                sound.0.toggle_playback();
            }
        } else if let Ok(sound) = antidote_sound.single() {
            sound.0.stop();
            commands.entity(sound.1).despawn();
        }
    }
}
