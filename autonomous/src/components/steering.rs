use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use super::physics::Physical;

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

#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub enum SteeringBehavior {
    Seek(Vec2),
    Flee(Vec2),
    Arrive(Vec2, Deceleration),
    // TODO: this is an Option because Inspectable requires Default
    // is there a better way to do this to avoid needing it to be wrapped?
    Pursuit(Option<Entity>),
    Idle,
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        Self::Idle
    }
}

impl SteeringBehavior {
    pub fn force(
        &self,
        physical: &Physical,
        transform: &Transform,
        query: &Query<(Entity, &SteeringBehavior, &mut Physical, &Transform)>,
    ) -> Vec2 {
        match self {
            Self::Seek(target) => self.seek_force(*target, physical, transform),
            Self::Flee(target) => self.flee_force(*target, physical, transform),
            Self::Arrive(target, deceleration) => {
                self.arrive_force(*target, *deceleration, physical, transform)
            }
            Self::Pursuit(target) => match target {
                Some(target) => self.pursuit_force(*target, physical, transform, query),
                None => Vec2::ZERO,
            },
            Self::Idle => Vec2::ZERO,
        }
    }

    fn seek_force(&self, target: Vec2, physical: &Physical, transform: &Transform) -> Vec2 {
        let translation = transform.translation.truncate();

        let desired_velocity = (target - translation).normalize_or_zero() * physical.max_speed;
        desired_velocity - physical.velocity
    }

    fn flee_force(&self, target: Vec2, physical: &Physical, transform: &Transform) -> Vec2 {
        let translation = transform.translation.truncate();

        let panic_distance_squared = 100.0 * 100.0;
        if translation.distance_squared(target) > panic_distance_squared {
            return Vec2::ZERO;
        }

        let desired_velocity = (translation - target).normalize_or_zero() * physical.max_speed;
        desired_velocity - physical.velocity
    }

    fn arrive_force(
        &self,
        target: Vec2,
        deceleration: Deceleration,
        physical: &Physical,
        transform: &Transform,
    ) -> Vec2 {
        let translation = transform.translation.truncate();
        let deceleration = deceleration as i32;

        let to_target = target - translation;

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

    fn pursuit_force(
        &self,
        target: Entity,
        physical: &Physical,
        transform: &Transform,
        query: &Query<(Entity, &SteeringBehavior, &mut Physical, &Transform)>,
    ) -> Vec2 {
        if let Ok((_, _, evader_physical, evader_transform)) = query.get(target) {
            let to_evader = (evader_transform.translation - transform.translation).truncate();
            let relative_heading = physical.heading.dot(evader_physical.heading);

            // if the evader is ahead and facing us, we can just seek it
            if to_evader.dot(physical.heading) > 0.0 && relative_heading < -0.95 {
                return self.seek_force(
                    evader_transform.translation.truncate(),
                    physical,
                    transform,
                );
            }

            // not ahead, so predict future position and seek that
            // look-ahead time is proportional to the distance between the evader
            // and us; and is inversly proportional to the sum of our velocities
            let look_ahead_time =
                to_evader.length() / (physical.max_speed + evader_physical.speed());

            return self.seek_force(
                evader_transform.translation.truncate()
                    + evader_physical.velocity * look_ahead_time,
                physical,
                transform,
            );
        }

        warn!("steering pursuit has invalid target!");
        Vec2::ZERO
    }
}
