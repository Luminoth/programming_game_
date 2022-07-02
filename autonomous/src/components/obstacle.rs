use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::util::rotate_around_origin;

#[derive(Debug, Default, Component)]
pub struct Obstacle;

#[derive(Debug, Default, Component)]
pub struct ObstacleDebug;

#[derive(Debug, Default, Component, Inspectable)]
pub struct ObstacleAvoidance {
    pub box_length: f32,
}

#[derive(Debug, Default, Component)]
pub struct ObstacleAvoidanceDebug;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Wall {
    pub extents: Vec2,

    // the wall normal
    pub facing: Vec2,
}

impl Wall {
    pub fn from(&self, position: Vec2) -> Vec2 {
        position - self.extents * 0.5
    }

    pub fn to(&self, position: Vec2) -> Vec2 {
        position + self.extents * 0.5
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct WallAvoidance {
    pub feelers: [Vec2; 3],
}

impl WallAvoidance {
    pub fn create_feelers(&mut self, position: Vec2, heading: Vec2, feeler_length: f32) {
        // straight ahead
        self.feelers[0] = position + feeler_length * heading;

        // left
        let temp = rotate_around_origin(heading, std::f32::consts::FRAC_PI_2 * 3.5);
        self.feelers[1] = position + feeler_length * 0.5 * temp;

        // right
        let temp = rotate_around_origin(heading, std::f32::consts::FRAC_2_PI * 0.5);
        self.feelers[2] = position + feeler_length * 0.5 * temp;
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct WallQuery<'w> {
    pub wall: &'w Wall,
    pub transform: &'w Transform,
}

// TODO: hide
