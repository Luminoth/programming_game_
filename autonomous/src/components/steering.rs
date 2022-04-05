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
    Idle,
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        Self::Idle
    }
}

impl SteeringBehavior {
    pub fn force(&self, physical: &Physical, transform: &Transform) -> Vec2 {
        match self {
            Self::Seek(target) => self.seek_force(*target, physical, transform),
            Self::Flee(target) => self.flee_force(*target, physical, transform),
            Self::Arrive(target, deceleration) => {
                self.arrive_force(*target, *deceleration, physical, transform)
            }
            Self::Idle => Vec2::ZERO,
        }
    }

    fn seek_force(&self, target: Vec2, physical: &Physical, transform: &Transform) -> Vec2 {
        let translation = transform.translation.truncate();

        let desired_velocity = (target - translation).normalize() * physical.max_speed;
        desired_velocity - physical.velocity
    }

    fn flee_force(&self, target: Vec2, physical: &Physical, transform: &Transform) -> Vec2 {
        let translation = transform.translation.truncate();

        let panic_distance_squared = 100.0 * 100.0;
        if translation.distance_squared(target) > panic_distance_squared {
            return Vec2::ZERO;
        }

        let desired_velocity = (translation - target).normalize() * physical.max_speed;
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
}
