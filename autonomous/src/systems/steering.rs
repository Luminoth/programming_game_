use std::collections::HashMap;

use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;

pub fn update_seek(mut query: Query<(&Seek, &SeekTarget, &mut Physical, &Transform)>) {
    for (steering, target, mut physical, transform) in query.iter_mut() {
        let force = steering.force(target, &physical, transform);
        physical.apply_force(force);
    }
}

pub fn update_flee(mut query: Query<(&Flee, &FleeTarget, &mut Physical, &Transform)>) {
    for (steering, target, mut physical, transform) in query.iter_mut() {
        let force = steering.force(target, &physical, transform);
        physical.apply_force(force);
    }
}

pub fn update_arrive(mut query: Query<(&Arrive, &ArriveTarget, &mut Physical, &Transform)>) {
    for (steering, target, mut physical, transform) in query.iter_mut() {
        let force = steering.force(target, &physical, transform);
        physical.apply_force(force);
    }
}

pub fn update_pursuit(
    pursuers: Query<(Entity, &Pursuit, &PursuitTarget)>,
    mut entities: Query<(&mut Physical, &Transform)>,
) {
    let mut forces = HashMap::new();

    // first pass, construct the set of force changes for each pursuer
    for (entity, steering, target) in pursuers.iter() {
        if let Ok((physical, transform)) = entities.get(entity) {
            let steering_force = steering.force(target, physical, transform, &entities);

            let force = forces.entry(entity).or_insert_with(Vec2::default);
            *force += steering_force;
        }
    }

    // second pass, apply the set of force changes for each entity
    for (entity, _, _) in pursuers.iter() {
        if let Ok((mut physical, _)) = entities.get_mut(entity) {
            let force = forces.entry(entity).or_insert_with(Vec2::default);
            physical.apply_force(*force);
        }
    }
}

pub fn update_evade(
    evaders: Query<(Entity, &Evade, &EvadeTarget)>,
    mut entities: Query<(&mut Physical, &Transform)>,
) {
    let mut forces = HashMap::new();

    // first pass, construct the set of force changes for each entity
    for (entity, steering, target) in evaders.iter() {
        if let Ok((physical, transform)) = entities.get(entity) {
            let steering_force = steering.force(target, physical, transform, &entities);

            let force = forces.entry(entity).or_insert_with(Vec2::default);
            *force += steering_force;
        }
    }

    // second pass, apply the set of force changes for each entity
    for (entity, _, _) in evaders.iter() {
        if let Ok((mut physical, _)) = entities.get_mut(entity) {
            let force = forces.entry(entity).or_insert_with(Vec2::default);
            physical.apply_force(*force);
        }
    }
}

pub fn update_wander(mut query: Query<(&mut Wander, &mut Physical)>) {
    for (mut steering, mut physical) in query.iter_mut() {
        let force = steering.force();
        physical.apply_force(force);
    }
}
