use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use rand::Rng;

use crate::util::point_to_world_space;

use super::physics::*;

pub trait SteeringBehavior: std::fmt::Debug + Component {}

fn seek_force(target: Vec2, physical: &PhysicalQueryMutItem) -> Vec2 {
    let translation = physical.transform.translation.truncate();

    let desired_velocity = (target - translation).normalize_or_zero() * physical.physical.max_speed;
    desired_velocity - physical.physical.velocity
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct SeekTarget {
    pub position: Vec2,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Seek;

impl SteeringBehavior for Seek {}

impl Seek {
    pub fn force(&self, target: &SeekTarget, physical: &PhysicalQueryMutItem) -> Vec2 {
        seek_force(target.position, physical)
    }
}

#[derive(WorldQuery)]
pub struct SeekQuery<'w> {
    pub steering: &'w Seek,
    pub target: &'w SeekTarget,
}

fn flee_force(target: Vec2, physical: &PhysicalQueryMutItem) -> Vec2 {
    let translation = physical.transform.translation.truncate();

    let panic_distance_squared = 100.0 * 100.0;
    if translation.distance_squared(target) > panic_distance_squared {
        return Vec2::ZERO;
    }

    let desired_velocity = (translation - target).normalize_or_zero() * physical.physical.max_speed;
    desired_velocity - physical.physical.velocity
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
    pub fn force(&self, target: &FleeTarget, physical: &PhysicalQueryMutItem) -> Vec2 {
        flee_force(target.position, physical)
    }
}

#[derive(WorldQuery)]
pub struct FleeQuery<'w> {
    pub steering: &'w Flee,
    pub target: &'w FleeTarget,
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
    pub fn force(&self, target: &ArriveTarget, physical: &PhysicalQueryMutItem) -> Vec2 {
        let translation = physical.transform.translation.truncate();
        let deceleration = self.deceleration as i32;

        let to_target = target.position - translation;

        let dist = to_target.length();
        if dist > 0.0 {
            // fine tweaking of deceleration
            let deceleration_tweaker = 0.3;

            let speed = (dist / (deceleration as f32 * deceleration_tweaker))
                .min(physical.physical.max_speed);
            let desired_velocity = to_target * speed / dist;
            return desired_velocity - physical.physical.velocity;
        }

        Vec2::ZERO
    }
}

#[derive(WorldQuery)]
pub struct ArriveQuery<'w> {
    pub steering: &'w Arrive,
    pub target: &'w ArriveTarget,
}

fn turnaround_time(target: Vec2, physical: &PhysicalQueryMutItem) -> f32 {
    let to_target = (target - physical.transform.translation.truncate()).normalize_or_zero();
    let dot = physical.physical.heading.dot(to_target);

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
        pursuer: Entity,
        target: &PursuitTarget,
        entities: &mut Query<PhysicalQueryMut>,
    ) -> Vec2 {
        if let Ok([pursuer, evader]) = entities.get_many_mut([pursuer, target.entity]) {
            let to_evader =
                (evader.transform.translation - pursuer.transform.translation).truncate();
            let relative_heading = pursuer.physical.heading.dot(evader.physical.heading);

            // if the evader is ahead and facing us, we can just seek it
            if to_evader.dot(pursuer.physical.heading) > 0.0 && relative_heading < -0.95 {
                return seek_force(evader.transform.translation.truncate(), &pursuer);
            }

            // not ahead, so predict future position and seek that
            // look-ahead time is proportional to the distance between the evader
            // and us; and is inversly proportional to the sum of our velocities
            // TODO: zero check this
            let mut look_ahead_time =
                to_evader.length() / (pursuer.physical.max_speed + evader.physical.speed());

            look_ahead_time += turnaround_time(evader.transform.translation.truncate(), &pursuer);

            return seek_force(
                evader.transform.translation.truncate()
                    + evader.physical.velocity * look_ahead_time,
                &pursuer,
            );
        }

        warn!("pursuit has invalid target!");
        Vec2::ZERO
    }
}

#[derive(WorldQuery)]
pub struct PursuitQuery<'w> {
    pub steering: &'w Pursuit,
    pub target: &'w PursuitTarget,
}

// TODO: offset pursuit

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
        evader: Entity,
        target: &EvadeTarget,
        entities: &mut Query<PhysicalQueryMut>,
    ) -> Vec2 {
        // TODO: if the target the evader is evading is on top of it
        // (to_pursuer.length() == 0) then the evader won't try to evade

        if let Ok([evader, pursuer]) = entities.get_many_mut([evader, target.entity]) {
            let to_pursuer =
                (pursuer.transform.translation - evader.transform.translation).truncate();

            // look-ahead time is proportional to the distance between the pursuer
            // and us; and is inversly proportional to the sum of our velocities
            // TODO: zero check this
            let look_ahead_time =
                to_pursuer.length() / (evader.physical.max_speed + pursuer.physical.speed());

            return flee_force(
                pursuer.transform.translation.truncate()
                    + pursuer.physical.velocity * look_ahead_time,
                &evader,
            );
        }

        warn!("evade has invalid target!");
        Vec2::ZERO
    }
}

#[derive(WorldQuery)]
pub struct EvadeQuery<'w> {
    pub steering: &'w Evade,
    pub target: &'w EvadeTarget,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Wander {
    pub radius: f32,
    pub distance: f32,
    pub jitter: f32,

    target: Vec2,
}

impl SteeringBehavior for Wander {}

impl Wander {
    // distance also determines the general direction we wander in
    pub fn new(radius: f32, distance: f32, jitter: f32) -> Self {
        Self {
            radius,
            distance,
            jitter,
            target: Vec2::ZERO,
        }
    }

    pub fn force(&mut self, physical: &PhysicalQueryMutItem) -> Vec2 {
        let mut rng = rand::thread_rng();

        // add some jitter to the target
        self.target += Vec2::new(
            rng.gen_range(-1.0..=1.0) * self.jitter,
            rng.gen_range(-1.0..=1.0) * self.jitter,
        );

        // extend out to the circle radius
        self.target = self.target.normalize_or_zero() * self.radius;

        let translation = physical.transform.translation.truncate();

        // project into world space
        let local = self.target + Vec2::new(self.distance, 0.0);
        let world = point_to_world_space(
            local,
            physical.physical.heading,
            physical.physical.side,
            translation,
        );

        // move the target out front
        world - translation
    }
}

// TODO: interpose

// TODO: path follow
