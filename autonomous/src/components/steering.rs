use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use super::physics::Physical;

#[derive(Debug, Component, Inspectable)]
pub enum SteeringBehavior {
    Seek(Vec2),
    Flee(Vec2),
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
}
