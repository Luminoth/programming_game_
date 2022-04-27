use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use super::physics::Physical;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Ball;

impl Ball {
    pub fn kick(&self, physical: &mut Physical, direction: Vec2, force: f32) {
        let direction = direction.normalize();
        physical.apply_force(direction * force);
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct BallQuery<'w> {
    pub ball: &'w Ball,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct BallQueryMut<'w> {
    pub ball: &'w mut Ball,
}
