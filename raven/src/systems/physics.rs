use bevy::prelude::*;

use crate::components::physics::*;

pub fn update(_time: Res<Time>, mut physicals: Query<PhysicalQueryUpdateMut>) {
    for mut physical in physicals.iter_mut() {
        physical
            .physical
            .update(&mut physical.transform /*, time.delta_seconds()*/);
    }
}
