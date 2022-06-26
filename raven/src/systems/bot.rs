use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::physics::*;
use crate::components::wall::*;
use crate::ORTHO_SIZE;

pub fn check_bounds(
    windows: Res<Windows>,
    mut bots: Query<(PhysicalQueryMut, &Bounds), With<Bot>>,
) {
    let window = windows.get_primary().unwrap();
    let aspect_ratio = window.width() / window.height();

    let max_x = ORTHO_SIZE;
    let max_y = ORTHO_SIZE / aspect_ratio;

    for (mut physical, bounds) in bots.iter_mut() {
        // TODO: need to account for bot bounds in check

        if physical.physical.cache.future_position.x < -max_x
            || physical.physical.cache.future_position.x > max_x
            || physical.physical.cache.future_position.y < -max_y
            || physical.physical.cache.future_position.y > max_y
        {
            // TODO: we should stop at the intersection
            physical.physical.stop();
        }
    }
}

pub fn check_wall_collision(
    mut bots: Query<(PhysicalQueryMut, &Bounds), With<Bot>>,
    walls: Query<(&Transform, &Bounds), With<Wall>>,
) {
    for (mut physical, bounds) in bots.iter_mut() {
        // TODO: need to account for bot bounds in raycast

        for (wall_transform, wall_bounds) in walls.iter() {
            let wall_position = wall_transform.translation.truncate();

            let contains = wall_bounds.contains(wall_position, physical.physical.cache.position);
            if contains {
                // TODO: push back out of the wall?
                continue;
            }

            if let Some(hit) = wall_bounds.ray_intersects(
                wall_position,
                physical.physical.cache.position,
                physical.physical.cache.heading,
                physical.physical.cache.max_distance,
            ) {
                // TODO: we should stop at the intersection
                physical.physical.stop();
            }
        }
    }
}
