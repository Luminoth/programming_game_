use bevy::prelude::*;
use bevy_inspector_egui::*;
use rand::Rng;

use crate::resources::*;

use super::physics::Physical;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Ball;

impl Ball {
    pub fn add_noise_to_kick(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        target: Vec2,
    ) -> Vec2 {
        let mut rng = rand::thread_rng();

        let displacement = (std::f32::consts::PI
            - std::f32::consts::PI * params.player_kick_accuracy)
            * rng.gen_range(-1.0..=1.0);

        let to_target = target - displacement;

        to_target + transform.translation.truncate()
    }

    pub fn kick(&self, physical: &mut Physical, direction: Vec2, force: f32) {
        let direction = direction.normalize();
        physical.apply_force(direction * force);
    }
}
