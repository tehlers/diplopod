mod systems;

use crate::systems::hello_world::hello_world;
use bevy::prelude::*;

fn main() {
    App::build().add_system(hello_world.system()).run();
}
