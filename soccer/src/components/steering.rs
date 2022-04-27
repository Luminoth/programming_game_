use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use super::physics::*;

pub trait SteeringBehavior: std::fmt::Debug + Component {}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Steering {
    pub target: Vec2,
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

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SteeringQueryMut<'w> {
    pub steering: &'w mut Steering,
    pub physical: &'w mut Physical,
}

fn seek_force(target: Vec2, physical: &PhysicalQueryItem) -> Vec2 {
    let translation = physical.transform.translation.truncate();

    let desired_velocity = (target - translation).normalize_or_zero() * physical.physical.max_speed;
    desired_velocity - physical.physical.velocity
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Seek;

impl SteeringBehavior for Seek {}

impl Seek {
    pub fn force(&self, steering: &Steering, physical: &PhysicalQueryItem) -> Vec2 {
        seek_force(steering.target, physical)
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SeekQueryMut<'w> {
    pub seek: &'w Seek,
    pub steering: &'w mut Steering,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct ObstacleAvoidance {
    pub box_length: f32,
}

#[derive(Debug, Default, Component)]
pub struct ObstacleAvoidanceDebug;
