use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn movement(mut heads: Query<(Entity, &DiplopodHead)>, mut positions: Query<&mut Position>) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head_pos.x += head.direction.x as i32;
        head_pos.y += head.direction.y as i32;
    }
}
