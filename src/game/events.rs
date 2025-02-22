use bevy::{ecs::event::Event, prelude::Transform};

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct SpawnConsumables {
    pub regular: bool,
}

#[derive(Event)]
pub struct ShowMessage {
    pub text: String,
    pub transform: Transform,
}
