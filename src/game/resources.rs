use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::components::{ConsumablePosition, Position};

#[derive(Default, Resource)]
pub struct DiplopodSegments(pub Vec<Entity>);

#[derive(Clone, Resource)]
pub struct FreeConsumablePositions {
    pub positions: Vec<ConsumablePosition>,
    width: i32,
    height: i32,
}

impl FreeConsumablePositions {
    pub fn new(width: i32, height: i32) -> Self {
        let positions = Self::new_positions(width, height);
        Self {
            positions,
            width,
            height,
        }
    }

    fn new_positions(width: i32, height: i32) -> Vec<ConsumablePosition> {
        let mut positions = Vec::new();

        for x in 0..width {
            for y in 0..height {
                positions.push(ConsumablePosition { x, y });
            }
        }

        positions.shuffle(&mut thread_rng());

        return positions;
    }

    pub fn shuffle(&mut self) {
        self.positions.shuffle(&mut thread_rng());
    }

    pub fn remove(&mut self, position: &ConsumablePosition) {
        self.positions.retain(|&p| p != *position);
    }

    pub fn remove_all(&mut self, positions: &Vec<ConsumablePosition>) {
        for position in positions {
            self.remove(position);
        }
    }

    pub fn reset(&mut self) {
        self.positions = Self::new_positions(self.width, self.height);
    }
}

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Default, Resource)]
pub struct LastSpecialSpawn(pub u32);

#[derive(Default, Resource)]
pub struct ImmunityTime(pub u8);

#[derive(Default, Debug, Resource)]
pub struct TileSize(pub i32);

#[derive(Default, Debug, Resource)]
pub struct UpperLeft {
    pub x: i32,
    pub y: i32,
}

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
pub struct Fonts {
    pub regular: Handle<Font>,
}

#[derive(Default, Resource)]
pub struct Highscore(pub u16);

#[derive(Default, Resource)]
pub struct Lastscore(pub u16);