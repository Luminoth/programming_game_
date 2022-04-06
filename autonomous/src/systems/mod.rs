pub mod debug;
pub mod physics;
pub mod steering;

use bevy::prelude::*;

use crate::components::physics::*;

pub fn wrap(window: Res<WindowDescriptor>, mut query: Query<&mut Transform>) {
    let half_width = window.width / 2.0;
    let half_height = window.height / 2.0;

    for mut transform in query.iter_mut() {
        if transform.translation.x < -half_width {
            transform.translation.x = half_width;
        } else if transform.translation.x > half_width {
            transform.translation.x = -half_width;
        }

        if transform.translation.y < -half_height {
            transform.translation.y = half_height;
        } else if transform.translation.y > half_height {
            transform.translation.y = -half_height;
        }
    }
}

pub fn facing(_time: Res<Time>, mut query: Query<(&mut Transform, &Physical)>) {
    // https://github.com/bevyengine/bevy/issues/2041
    let dt = PHYSICS_STEP;

    for (mut transform, physical) in query.iter_mut() {
        if physical.velocity.length_squared() < std::f32::EPSILON {
            continue;
        }

        let angle = -physical.velocity.normalize_or_zero().angle_between(Vec2::Y);
        transform.rotation = transform.rotation.slerp(Quat::from_rotation_z(angle), dt);
    }
}
