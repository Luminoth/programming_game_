use bevy::prelude::*;

use super::physics::Physical;

#[derive(Debug, Component)]
pub enum SteeringBehavior {
    Seek(Vec2),
    Test,
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        Self::Test
    }
}

impl SteeringBehavior {
    pub fn force(&self, physical: &Physical, transform: &Transform) -> Vec2 {
        match self {
            Self::Seek(target) => self.seek_force(*target, physical, transform),
            Self::Test => self.test_force(),
        }
    }

    fn seek_force(&self, target: Vec2, physical: &Physical, transform: &Transform) -> Vec2 {
        let desired_velocity =
            (target - transform.translation.truncate()).normalize() * physical.max_speed;
        desired_velocity - physical.velocity
    }

    fn test_force(&self) -> Vec2 {
        Vec2::new(10.0, 10.0)
    }
}
