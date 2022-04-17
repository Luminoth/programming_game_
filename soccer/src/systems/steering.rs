use bevy::prelude::*;

use crate::components::steering::*;

pub fn update(mut query: Query<SteeringQueryMut>) {
    for mut steering in query.iter_mut() {
        steering.steering.update(&mut steering.physical);
    }
}
