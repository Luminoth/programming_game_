use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use super::physics::Physical;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Steering {
    pub accumulated_force: Vec2,
}

impl Steering {
    #[allow(dead_code)]
    pub fn accumulate_force(&mut self, physical: &Physical, force: Vec2, weight: f32) {
        let force = force * weight;

        let mag_so_far = self.accumulated_force.length();
        let mag_remain = physical.max_force - mag_so_far;
        if mag_remain <= 0.0 {
            return;
        }

        let to_add = force.length();
        self.accumulated_force += if to_add < mag_remain {
            force
        } else {
            force.normalize_or_zero() * mag_remain
        };
    }

    pub fn update(&mut self, physical: &mut Physical) {
        physical.apply_force(self.accumulated_force);
        self.accumulated_force = Vec2::ZERO;
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct ObstacleAvoidance {
    pub box_length: f32,
}

#[derive(Debug, Default, Component)]
pub struct ObstacleAvoidanceDebug;
