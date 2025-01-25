use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::components::Position;

#[derive(Default, Resource)]
pub struct DiplopodSegments(pub Vec<Entity>);

#[derive(Clone, Resource)]
pub struct FreePositions {
    pub positions: Vec<Position>,
    width: i32,
    height: i32,
}

impl FreePositions {
    pub fn new(width: i32, height: i32) -> Self {
        let positions = Self::new_positions(width, height);
        Self {
            positions,
            width,
            height,
        }
    }

    fn new_positions(width: i32, height: i32) -> Vec<Position> {
        let mut positions = Vec::new();

        for x in 0..width {
            for y in 0..height {
                positions.push(Position { x, y });
            }
        }

        positions.shuffle(&mut thread_rng());

        positions
    }

    pub fn shuffle(&mut self) {
        self.positions.shuffle(&mut thread_rng());
    }

    pub fn remove(&mut self, position: &Position) {
        self.positions.retain(|&p| p != *position);
    }

    pub fn remove_all(&mut self, positions: &Vec<Position>) {
        for position in positions {
            self.remove(position);
        }
    }

    pub fn reset(&mut self) {
        self.positions = Self::new_positions(self.width, self.height);
    }
}

#[derive(Default, Resource)]
pub struct LastSpecialSpawn(pub u32);

#[derive(Default, Resource)]
pub struct ImmunityTime(pub u8);

#[derive(Resource)]
pub struct Sounds {
    pub eat_food: Handle<AudioSource>,
    pub eat_poison: Handle<AudioSource>,
    pub special_spawn: Handle<AudioSource>,
    pub super_food: Handle<AudioSource>,
    pub antidote: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct DefaultFontHandle(pub Handle<Font>);

#[derive(Default, Resource)]
pub struct Highscore(pub u16);

#[derive(Default, Resource)]
pub struct Lastscore(pub u16);

#[derive(Default, Resource)]
pub struct Paused;
