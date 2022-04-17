use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use super::physics::Physical;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Ball;

#[derive(WorldQuery)]
pub struct BallQuery<'w> {
    pub ball: &'w Ball,
    pub transform: &'w Transform,
    pub physical: &'w Physical,
}
