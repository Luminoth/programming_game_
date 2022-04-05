use bevy::prelude::*;

#[derive(Debug, Component)]
pub enum SteeringBehavior {
    Test,
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        Self::Test
    }
}

impl SteeringBehavior {
    pub fn force(&self) -> Vec2 {
        match self {
            Self::Test => self.test_force(),
        }
    }

    fn test_force(&self) -> Vec2 {
        Vec2::new(10.0, 0.0)
    }
}
