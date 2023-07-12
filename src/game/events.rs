use crate::game::components::ConsumablePosition;
use bevy::ecs::event::Event;

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct Growth(pub u8);

#[derive(Event)]
pub struct SpawnConsumables {
    pub regular: bool,
    pub new_segments: u8,
}

#[derive(Event)]
pub struct ShowMessage {
    pub text: String,
    pub position: ConsumablePosition,
}
