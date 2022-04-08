use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use super::physics::Physical;

pub trait SteeringBehavior: std::fmt::Debug + Component {}

fn seek_force(target: Vec2, physical: &Physical, transform: &Transform) -> Vec2 {
    let translation = transform.translation.truncate();

    let desired_velocity = (target - translation).normalize_or_zero() * physical.max_speed;
    desired_velocity - physical.velocity
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct SeekTarget {
    pub position: Vec2,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Seek;

impl SteeringBehavior for Seek {}

impl Seek {
    pub fn force(&self, target: &SeekTarget, physical: &Physical, transform: &Transform) -> Vec2 {
        seek_force(target.position, physical, transform)
    }
}

fn flee_force(target: Vec2, physical: &Physical, transform: &Transform) -> Vec2 {
    let translation = transform.translation.truncate();

    let panic_distance_squared = 100.0 * 100.0;
    if translation.distance_squared(target) > panic_distance_squared {
        return Vec2::ZERO;
    }

    let desired_velocity = (translation - target).normalize_or_zero() * physical.max_speed;
    desired_velocity - physical.velocity
}

// TODO: if the agent spawns on top of the postion its fleeing
// something breaks the numbers (NaN), but I'm not sure what / where
#[derive(Debug, Default, Component, Inspectable)]
pub struct FleeTarget {
    pub position: Vec2,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Flee;

impl SteeringBehavior for Flee {}

impl Flee {
    pub fn force(&self, target: &FleeTarget, physical: &Physical, transform: &Transform) -> Vec2 {
        flee_force(target.position, physical, transform)
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct ArriveTarget {
    pub position: Vec2,
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub enum Deceleration {
    Slow = 3,
    Normal = 2,
    Fast = 1,
}

impl Default for Deceleration {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Arrive {
    pub deceleration: Deceleration,
}

impl SteeringBehavior for Arrive {}

impl Arrive {
    pub fn force(&self, target: &ArriveTarget, physical: &Physical, transform: &Transform) -> Vec2 {
        let translation = transform.translation.truncate();
        let deceleration = self.deceleration as i32;

        let to_target = target.position - translation;

        let dist = to_target.length();
        if dist > 0.0 {
            // fine tweaking of deceleration
            let deceleration_tweaker = 0.3;

            let speed =
                (dist / (deceleration as f32 * deceleration_tweaker)).min(physical.max_speed);
            let desired_velocity = to_target * speed / dist;
            return desired_velocity - physical.velocity;
        }

        Vec2::ZERO
    }
}

fn turnaround_time(target: Vec2, physical: &Physical, transform: &Transform) -> f32 {
    let to_target = (target - transform.translation.truncate()).normalize_or_zero();
    let dot = physical.heading.dot(to_target);

    // adjust to get ~1 second for 180 turn
    // higher max turn means higher coefficient
    let coefficient = 0.5;

    // dot == 1.0 if ahead, -1.0 if behind
    // this should give a value proportional to our rotational displacement
    (dot - 1.0) * -coefficient
}

#[derive(Debug, Component, Inspectable)]
pub struct PursuitTarget {
    pub entity: Entity,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Pursuit;

impl SteeringBehavior for Pursuit {}

impl Pursuit {
    pub fn force(
        &self,
        target: &PursuitTarget,
        physical: &Physical,
        transform: &Transform,
        entities: &Query<(&mut Physical, &Transform)>,
    ) -> Vec2 {
        if let Ok((evader_physical, evader_transform)) = entities.get(target.entity) {
            let to_evader = (evader_transform.translation - transform.translation).truncate();
            let relative_heading = physical.heading.dot(evader_physical.heading);

            // if the evader is ahead and facing us, we can just seek it
            if to_evader.dot(physical.heading) > 0.0 && relative_heading < -0.95 {
                return seek_force(evader_transform.translation.truncate(), physical, transform);
            }

            // not ahead, so predict future position and seek that
            // look-ahead time is proportional to the distance between the evader
            // and us; and is inversly proportional to the sum of our velocities
            // TODO: zero check this
            let mut look_ahead_time =
                to_evader.length() / (physical.max_speed + evader_physical.speed());

            look_ahead_time +=
                turnaround_time(evader_transform.translation.truncate(), physical, transform);

            return seek_force(
                evader_transform.translation.truncate()
                    + evader_physical.velocity * look_ahead_time,
                physical,
                transform,
            );
        }

        warn!("pursuit has invalid target!");
        Vec2::ZERO
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct EvadeTarget {
    pub entity: Entity,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Evade;

impl SteeringBehavior for Evade {}

impl Evade {
    pub fn force(
        &self,
        target: &EvadeTarget,
        physical: &Physical,
        transform: &Transform,
        entities: &Query<(&mut Physical, &Transform)>,
    ) -> Vec2 {
        // TODO: if the target the evader is evading is on top of it
        // (to_pursuer.length() == 0) then the evader won't try to evade

        if let Ok((pursuer_physical, pursuer_transform)) = entities.get(target.entity) {
            let to_pursuer = (pursuer_transform.translation - transform.translation).truncate();

            // look-ahead time is proportional to the distance between the pursuer
            // and us; and is inversly proportional to the sum of our velocities
            // TODO: zero check this
            let look_ahead_time =
                to_pursuer.length() / (physical.max_speed + pursuer_physical.speed());

            return flee_force(
                pursuer_transform.translation.truncate()
                    + pursuer_physical.velocity * look_ahead_time,
                physical,
                transform,
            );
        }

        warn!("evade has invalid target!");
        Vec2::ZERO
    }
}
