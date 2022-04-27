use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;
use crate::resources::*;

pub fn update(mut query: Query<SteeringQueryMut>) {
    for mut steering in query.iter_mut() {
        steering.steering.update(&mut steering.physical);
    }
}

pub fn update_seek(params: Res<SimulationParams>, mut query: Query<(SeekQueryMut, PhysicalQuery)>) {
    for (mut steering, physical) in query.iter_mut() {
        let force = steering.seek.force(&steering.steering, &physical);
        steering
            .steering
            .accumulate_force(&physical.physical, force, params.seek_weight);
    }
}
