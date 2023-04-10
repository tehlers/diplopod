use crate::game::components::ConsumablePosition;

pub struct GameOver;

pub struct Growth(pub u8);

pub struct SpawnConsumables {
    pub regular: bool,
    pub new_segments: u8,
}

pub struct ShowMessage {
    pub text: String,
    pub position: ConsumablePosition,
}
