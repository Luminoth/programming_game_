use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::physics::*;
use crate::components::wall::*;
use crate::game::PHYSICS_STEP;

pub fn check_wall_collision(
    mut bots: Query<(PhysicalQueryMut, &Bounds), With<Bot>>,
    walls: Query<(&Transform, &Bounds), With<Wall>>,
) {
    for (mut physical, bounds) in bots.iter_mut() {
        let position = physical.transform.translation.truncate();
        let future_position = physical
            .physical
            .future_position(physical.transform, PHYSICS_STEP);

        let v = future_position - position;
        let distance = v.length();
        let direction = v.normalize_or_zero();

        // TODO: need to account for bot bounds in raycast

        for (wall_transform, wall_bounds) in walls.iter() {
            let wall_position = wall_transform.translation.truncate();

            if let Some(hit) =
                wall_bounds.ray_intersects(wall_position, position, direction, distance)
            {
                physical.physical.stop();
            }
        }
    }
}
