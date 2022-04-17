pub mod debug;
pub mod physics;
pub mod steering;

use bevy::prelude::*;

use crate::components::physics::*;

// TODO: this should exclude entities that don't move
pub fn wrap(window: Res<WindowDescriptor>, mut query: Query<PhysicalQuery>) {
    let half_width = window.width / 2.0;
    let half_height = window.height / 2.0;

    for mut physical in query.iter_mut() {
        if physical.transform.translation.x < -half_width {
            physical.transform.translation.x = half_width;
        } else if physical.transform.translation.x > half_width {
            physical.transform.translation.x = -half_width;
        }

        if physical.transform.translation.y < -half_height {
            physical.transform.translation.y = half_height;
        } else if physical.transform.translation.y > half_height {
            physical.transform.translation.y = -half_height;
        }
    }
}

pub fn facing(_time: Res<Time>, mut query: Query<PhysicalQuery>) {
    for mut physical in query.iter_mut() {
        if physical.physical.heading.length_squared() < std::f32::EPSILON {
            continue;
        }

        let angle = -physical.physical.heading.angle_between(Vec2::Y);
        physical.transform.rotation = Quat::from_rotation_z(angle);
    }
}
