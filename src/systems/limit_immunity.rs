use bevy::prelude::*;

use crate::resources::ImmunityTime;

pub fn limit_immunity(mut immunity_time: ResMut<ImmunityTime>) {
    if immunity_time.0 > 0 {
        immunity_time.0 -= 1;
    }
}
