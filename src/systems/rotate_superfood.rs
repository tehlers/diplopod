use bevy::prelude::*;

use crate::components::Superfood;

pub fn rotate_superfood(mut query: Query<&mut Transform, With<Superfood>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_seconds();
        transform.rotate(Quat::from_rotation_z(1.5 * delta));
    }
}
