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
        let bot_min_x =
            physical.physical.cache.future_position.x + bounds.center().x - bounds.width();
        let bot_min_y =
            physical.physical.cache.future_position.y + bounds.center().y - bounds.height();
        let bot_max_x =
            physical.physical.cache.future_position.x + bounds.center().x + bounds.width();
        let bot_max_y =
            physical.physical.cache.future_position.y + bounds.center().y + bounds.height();

        if bot_min_x < -max_x || bot_max_x > max_x || bot_min_y < -max_y || bot_max_y > max_y {
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
        // TODO: need to account for bounds width / height

        let bot_position = physical.physical.cache.position + bounds.center();
        let heading = physical.physical.cache.heading * physical.physical.cache.max_distance;

        for wall in walls.iter() {
            let wall_position = wall.transform.translation.truncate();
            let wall_from = wall.wall.from(wall_position);
            let wall_to = wall.wall.to(wall_position);

            /*let contains = wall_bounds.contains(wall_position, bot_position);
            if contains {
                // TODO: push back out of the wall?
                continue;
            }*/

            if line_intersection(wall_from, wall_to, bot_position, heading).is_some() {
                // TODO: we should stop at the intersection
                physical.physical.stop();
                break;
            }
        }
    }
}
