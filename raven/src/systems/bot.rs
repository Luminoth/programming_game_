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
    walls: Query<WallQuery>,
) {
    for (mut physical, bounds) in bots.iter_mut() {
        // TODO: need to account for bot bounds in raycast

        for wall in walls.iter() {
            let wall_position = wall.transform.translation.truncate();
            let wall_from = wall.wall.from(wall_position);
            let wall_to = wall.wall.to(wall_position);

            /*let contains = wall_bounds.contains(wall_position, physical.physical.cache.position);
            if contains {
                // TODO: push back out of the wall?
                continue;
            }*/

            if line_intersection(
                wall_from,
                wall_to,
                physical.physical.cache.position,
                physical.physical.cache.heading * physical.physical.cache.max_distance,
            )
            .is_some()
            {
                // TODO: we should stop at the intersection
                physical.physical.stop();
            }
        }
    }
}
