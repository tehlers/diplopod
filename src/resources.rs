use bevy::prelude::*;

pub struct Materials {
    pub diplopod_material: Handle<ColorMaterial>,
}

#[derive(Default)]
pub struct DiplopodSegments(pub Vec<Entity>);
