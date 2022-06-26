use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;
use crate::game::DEBUG_SORT;
use crate::util::*;

pub fn update(mut steering_behaviors: Query<SteeringQueryMut>) {
    for mut steering in steering_behaviors.iter_mut() {
        steering.steering.update_physical(&mut steering.physical);
    }
}

pub fn update_debug(
    agents: Query<(&Steering, &Children), Without<SteeringTargetDebug>>,
    mut steering_debug: Query<TransformQueryMut, With<SteeringTargetDebug>>,
) {
    for (steering, children) in agents.iter() {
        for &child in children.iter() {
            if let Ok(mut transform) = steering_debug.get_mut(child) {
                transform.transform.set_world_translation(
                    transform.global_transform,
                    steering.target.extend(DEBUG_SORT),
                );
            }
        }
    }
}

pub fn update_seek(mut seeking: Query<(SeekQueryMut, PhysicalQuery)>) {
    for (mut steering, physical) in seeking.iter_mut() {
        let force = steering.seek.force(&steering.steering, &physical);
        steering
            .steering
            .accumulate_force(physical.physical, force, 1.0);
    }
}

pub fn update_arrive(mut arriving: Query<(ArriveQueryMut, PhysicalQuery)>) {
    for (mut steering, physical) in arriving.iter_mut() {
        let force = steering.arrive.force(&steering.steering, &physical);
        steering
            .steering
            .accumulate_force(physical.physical, force, 1.0);
    }
}
