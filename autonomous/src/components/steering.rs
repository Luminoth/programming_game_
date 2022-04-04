use bevy::prelude::*;

pub trait Steering {}

#[derive(Debug, Default, Component)]
pub struct SteeringTest;

impl Steering for SteeringTest {}
