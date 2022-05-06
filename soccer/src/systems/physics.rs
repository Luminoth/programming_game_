use bevy::prelude::*;

use crate::components::physics::*;

pub fn update(_time: Res<Time>, mut physicals: Query<PhysicalQueryUpdateMut>) {
    for mut physical in physicals.iter_mut() {
        physical
            .physical
            .update(&mut physical.transform /*, time.delta_seconds()*/);
    }
}

pub fn facing(_time: Res<Time>, mut query: Query<PhysicalQueryUpdateMut>) {
    for mut physical in query.iter_mut() {
        if physical.physical.heading.length_squared() < std::f32::EPSILON {
            continue;
        }

        let angle = -physical.physical.heading.angle_between(Vec2::Y);
        physical.transform.rotation = Quat::from_rotation_z(angle);
    }
}
