use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;

pub fn update_steering<T>(mut query: Query<(&T, &mut Physical)>)
where
    T: Steering + Component,
{
    for (steering, mut physical) in query.iter_mut() {
        physical.apply_force(steering.force());
    }
}
