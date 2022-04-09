use std::collections::HashMap;

use bevy::prelude::*;

use crate::components::actor::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::resources::*;
use crate::util::*;

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

pub fn update_wander(mut query: Query<(&mut Wander, &mut Physical, &Transform)>) {
    for (mut steering, mut physical, transform) in query.iter_mut() {
        let force = steering.force(&physical, transform);
        physical.apply_force(force);
    }
}

pub fn update_obstacle_avoidance(
    params: Res<SimulationParams>,
    mut query: Query<(Entity, &Actor, &mut Physical, &Transform, &Name), With<ObstacleAvoidance>>,
    obstacles: Query<(Entity, &Actor, &Transform, &Name), With<Obstacle>>,
) {
    for (entity, actor, mut physical, transform, name) in query.iter_mut() {
        let box_length = params.min_detection_box_length
            + (physical.speed() / physical.max_speed) * params.min_detection_box_length;

        let mut closest_obstacle = None;
        let mut dist_to_closest = f32::MAX;
        for (obstacle, obstacle_actor, obstacle_transform, obstacle_name) in obstacles.iter() {
            // ignore ourself
            if obstacle == entity {
                continue;
            }

            // ignore anything out of range in front
            let to = obstacle_transform.translation - transform.translation;
            let range = obstacle_actor.bounding_radius + actor.bounding_radius;
            if to.length_squared() >= range * range {
                continue;
            }

            let obstacle_position = obstacle_transform.translation.truncate();
            let local_position = point_to_local_space(
                obstacle_position,
                physical.heading,
                physical.side,
                transform.translation.truncate(),
            );

            // ignore anything behind us
            if local_position.x < 0.0 {
                continue;
            }

            // ignore anything out of range above or below
            let expanded_radius = obstacle_actor.bounding_radius + actor.bounding_radius;
            if local_position.y >= expanded_radius {
                continue;
            }

            // line / circle intersection test
            let cx = local_position.x;
            let cy = local_position.y;
            let sqrt_part = (expanded_radius * expanded_radius - cy * cy).sqrt();
            let mut ip = cx - sqrt_part;
            if ip <= 0.0 {
                ip = cx + sqrt_part;
            }

            // is this the closest?
            if ip < dist_to_closest {
                dist_to_closest = ip;

                closest_obstacle = Some((
                    obstacle_name,
                    obstacle_position,
                    obstacle_actor.bounding_radius,
                    local_position,
                    dist_to_closest,
                ));
            }
        }

        // calculate the steering force
        if let Some((obstacle_name, position, radius, local_position, distance)) = closest_obstacle
        {
            info!(
                "{} avoiding obstacle {} at {} ({})",
                name.as_str(),
                obstacle_name.as_str(),
                position,
                distance
            );

            // the closer we are to the obstacle, the stronger the steering force
            let multiplier = 1.0 + (box_length - local_position.x) / box_length;

            let y = (radius - local_position.y) * multiplier;

            // apply a braking force proportional to the obstacle's distance
            let braking_weight = 0.2;
            let x = (radius - local_position.x) * braking_weight;

            let force = Vec2::new(x, y);

            let heading = physical.heading;
            let side = physical.side;
            physical.apply_force(vector_to_world_space(force, heading, side));
        }
    }
}
