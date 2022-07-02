use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::vehicle::*;
use crate::components::actor::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::resources::*;
use crate::util::*;

pub fn update_seek(params: Res<SimulationParams>, mut query: Query<(SeekQuery, PhysicalQueryMut)>) {
    for (steering, mut physical) in query.iter_mut() {
        let force = steering.steering.force(steering.target, &physical);
        physical
            .physical
            .accumulate_stearing_force(force, params.seek_weight);
    }
}

pub fn update_flee(params: Res<SimulationParams>, mut query: Query<(FleeQuery, PhysicalQueryMut)>) {
    for (steering, mut physical) in query.iter_mut() {
        let force = steering.steering.force(steering.target, &physical);
        physical
            .physical
            .accumulate_stearing_force(force, params.flee_weight);
    }
}

pub fn update_arrive(
    params: Res<SimulationParams>,
    mut query: Query<(ArriveQuery, PhysicalQueryMut)>,
) {
    for (steering, mut physical) in query.iter_mut() {
        let force = steering.steering.force(steering.target, &physical);
        physical
            .physical
            .accumulate_stearing_force(force, params.arrive_weight);
    }
}

pub fn update_pursuit(
    params: Res<SimulationParams>,
    mut pursuers: Query<(Entity, PursuitQuery)>,
    mut entities: Query<PhysicalQueryMut>,
) {
    for (entity, steering) in pursuers.iter_mut() {
        let force = steering
            .steering
            .force(entity, steering.target, &mut entities);
        if let Ok(mut physical) = entities.get_mut(entity) {
            physical
                .physical
                .accumulate_stearing_force(force, params.evade_weight);
        }
    }
}

pub fn update_evade(
    params: Res<SimulationParams>,
    mut evaders: Query<(Entity, EvadeQuery)>,
    mut entities: Query<PhysicalQueryMut>,
) {
    for (entity, steering) in evaders.iter_mut() {
        let force = steering
            .steering
            .force(entity, steering.target, &mut entities);
        if let Ok(mut physical) = entities.get_mut(entity) {
            physical
                .physical
                .accumulate_stearing_force(force, params.evade_weight);
        }
    }
}

pub fn update_wander(
    params: Res<SimulationParams>,
    mut query: Query<(&mut Wander, PhysicalQueryMut)>,
) {
    for (mut steering, mut physical) in query.iter_mut() {
        let force = steering.force(&physical);
        physical
            .physical
            .accumulate_stearing_force(force, params.wander_weight);
    }
}

// TODO: interpose

// TODO: path follow

pub fn update_obstacle_avoidance(
    params: Res<SimulationParams>,
    mut query: Query<(Entity, &mut Physical, &mut ObstacleAvoidance, &Children)>,
    obstacles: Query<Entity, With<Obstacle>>,
    actors: Query<(ActorQuery, &Transform)>,
    mut shapes: Query<&mut Path, With<ObstacleAvoidanceDebug>>,
) {
    for (entity, mut physical, mut avoidance, children) in query.iter_mut() {
        avoidance.box_length = params.min_detection_box_length
            + (physical.speed() / physical.max_speed) * params.min_detection_box_length;

        // update debug visual
        for child in children.iter() {
            if let Ok(mut path) = shapes.get_mut(*child) {
                *path = ShapePath::build_as(&shapes::Rectangle {
                    extents: Vec2::new(VEHICLE_RADIUS, avoidance.box_length),
                    origin: RectangleOrigin::CustomCenter(Vec2::new(
                        0.0,
                        avoidance.box_length * 0.5,
                    )),
                });
            }
        }

        // find the closest obstacle for avoidance
        let mut closest_obstacle = None;
        let mut dist_to_closest = f32::MAX;
        for obstacle in obstacles.iter() {
            // ignore ourself
            if obstacle == entity {
                continue;
            }

            let [(actor, transform), (obstacle_actor, obstacle_transform)] =
                actors.many([entity, obstacle]);

            // ignore anything out of range in front
            let to = obstacle_transform.translation - transform.translation;
            let range = avoidance.box_length + obstacle_actor.actor.bounding_radius;
            if to.length_squared() > range * range {
                continue;
            }

            // convert obstacle to local space
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
            let expanded_radius =
                obstacle_actor.actor.bounding_radius + actor.actor.bounding_radius;
            if local_position.y > expanded_radius {
                continue;
            }

            // line / circle intersection test (x = cX +/-sqrt(r^2-cY^2) for y=0)
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
                    obstacle_actor.name,
                    obstacle_position,
                    obstacle_actor.actor.bounding_radius,
                    local_position,
                    dist_to_closest,
                ));
            }
        }

        // calculate the steering force
        if let Ok((actor, _)) = actors.get(entity) {
            if let Some((obstacle_name, position, radius, local_position, distance)) =
                closest_obstacle
            {
                debug!(
                    "{} avoiding obstacle {} at {} ({})",
                    actor.name.as_str(),
                    obstacle_name.as_str(),
                    position,
                    distance
                );

                // the closer we are to the obstacle, the stronger the steering force
                let multiplier =
                    1.0 + (avoidance.box_length - local_position.x) / avoidance.box_length;

                let y = (radius - local_position.y) * multiplier;

                // apply a braking force proportional to the obstacle's distance
                let braking_weight = 0.2;
                let x = (radius - local_position.x) * braking_weight;

                let force = Vec2::new(x, y);

                let heading = physical.heading;
                let side = physical.side;
                physical.accumulate_stearing_force(
                    vector_to_world_space(force, heading, side),
                    params.obstacle_avoidance_weight,
                );
            }
        }
    }
}

pub fn update_wall_avoidance(
    params: Res<SimulationParams>,
    mut query: Query<(PhysicalQueryMut, &mut WallAvoidance, &Name), Without<Wall>>,
    walls: Query<WallQuery>,
) {
    for (mut physical, mut avoidance, name) in query.iter_mut() {
        let position = physical.transform.translation.truncate();

        avoidance.create_feelers(
            position,
            physical.physical.heading,
            params.wall_detection_feeler_length,
        );

        let mut steering_force = Vec2::ZERO;

        for feeler in avoidance.feelers {
            let mut dist_to_closest_ip = f32::MAX;
            let mut closest_wall_normal = None;
            let mut closest_point = Vec2::ZERO;

            for wall in walls.iter() {
                let wall_position = wall.transform.translation.truncate();

                if let Some((dist_to_this_ip, point)) = line_intersection(
                    position,
                    feeler,
                    wall.wall.from(wall_position),
                    wall.wall.to(wall_position),
                ) {
                    if dist_to_this_ip < dist_to_closest_ip {
                        dist_to_closest_ip = dist_to_this_ip;
                        closest_wall_normal = Some(wall.wall.facing);
                        closest_point = point;
                    }
                }
            }

            if let Some(closest_wall_normal) = closest_wall_normal {
                let overshoot = feeler - closest_point;
                steering_force += closest_wall_normal * overshoot.length();

                warn!(
                    "{} avoiding wall overshoot {}: {}",
                    name, overshoot, steering_force
                );
            }
        }

        physical
            .physical
            .accumulate_stearing_force(steering_force, params.wall_avoidance_weight);
    }
}

// TODO: hide
