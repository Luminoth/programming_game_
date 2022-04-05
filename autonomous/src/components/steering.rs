use bevy::prelude::*;

pub trait Steering: Component {
    fn force(&self) -> Vec2;
}

#[derive(Debug, Default, Component)]
pub struct SteeringTest;

impl Steering for SteeringTest {
    fn force(&self) -> Vec2 {
        Vec2::new(1.0, 0.0)
    }
}
