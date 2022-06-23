use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::physics::*;
use crate::components::wall::*;
use crate::game::PHYSICS_STEP;
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
        let future_position = physical
            .physical
            .future_position(physical.transform, PHYSICS_STEP);

        // TODO: need to account for bot bounds in check

        if future_position.x < -max_x
            || future_position.x > max_x
            || future_position.y < -max_y
            || future_position.y > max_y
        {
            physical.physical.stop();
        }
    }
}

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
