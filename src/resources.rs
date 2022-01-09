use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::components::{ConsumablePosition, Position};

#[derive(Default)]
pub struct DiplopodSegments(pub Vec<Entity>);

#[derive(Clone)]
pub struct FreeConsumablePositions {
    pub positions: Vec<ConsumablePosition>,
}

impl FreeConsumablePositions {
    pub fn new(width: i32, height: i32) -> Self {
        let mut positions = Vec::new();

        for x in 0..width {
            for y in 0..height {
                positions.push(ConsumablePosition { x, y });
            }
        }

        positions.shuffle(&mut thread_rng());

        Self { positions }
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
}

#[derive(Default)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Default)]
pub struct LastSpecialSpawn(pub u32);

#[derive(Default)]
pub struct ImmunityTime(pub u8);
