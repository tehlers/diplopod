use bevy::ecs::event::Event;

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct SpawnConsumables {
    pub regular: bool,
}
