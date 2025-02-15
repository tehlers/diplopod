use crate::game::components::DiplopodPosition;
use bevy::ecs::event::Event;

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct SpawnConsumables {
    pub regular: bool,
}

#[derive(Event)]
pub struct ShowMessage {
    pub text: String,
    pub position: DiplopodPosition,
}
