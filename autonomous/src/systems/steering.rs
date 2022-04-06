use std::collections::HashMap;

use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;

pub fn update_steering(mut query: Query<(Entity, &SteeringBehavior, &mut Physical, &Transform)>) {
    let mut forces = HashMap::new();

    // first pass, construct the set of force changes for each entity
    for (entity, steering, physical, transform) in query.iter() {
        let steering_force = steering.force(&physical, transform, &query);

        let force = forces.entry(entity).or_insert(Vec2::default());
        *force += steering_force;
    }

    // second pass, apply the set of force changes for each entity
    for (entity, _, mut physical, _) in query.iter_mut() {
        let force = forces.entry(entity).or_insert(Vec2::default());
        physical.apply_force(*force);
    }
}
