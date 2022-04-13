use bevy::prelude::*;

use crate::components::physics::Physical;

pub fn update(_time: Res<Time>, mut query: Query<(&mut Physical, &mut Transform)>) {
    for (mut physical, mut transform) in query.iter_mut() {
        physical.update(&mut transform /*, time.delta_seconds()*/);
    }
}
