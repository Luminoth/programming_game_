use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;

pub fn update_steering(mut query: Query<(&SteeringBehavior, &mut Physical, &Transform)>) {
    for (steering, mut physical, transform) in query.iter_mut() {
        let force = steering.force(&physical, transform);
        physical.apply_force(force);
    }
}
