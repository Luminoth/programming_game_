use bevy::prelude::*;

use crate::components::physics::*;

pub fn update(_time: Res<Time>, mut query: Query<PhysicalQueryMut>) {
    for mut physical in query.iter_mut() {
        physical
            .physical
            .update(&mut physical.transform /*, time.delta_seconds()*/);
    }
}

// TODO: non-penetration constrait
