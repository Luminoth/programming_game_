use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;

pub fn update_steering(mut query: Query<(&SteeringBehavior, &mut Physical)>) {
    for (steering, mut physical) in query.iter_mut() {
        physical.apply_force(steering.force());
    }
}
