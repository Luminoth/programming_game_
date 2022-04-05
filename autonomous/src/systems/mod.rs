pub mod debug;
pub mod physics;
pub mod steering;

use bevy::prelude::*;

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
