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
    pub from: Vec2,
    pub to: Vec2,

    // the wall normal
    normal: Vec2,
}

impl Wall {
    pub fn new(from: Vec2, to: Vec2) -> Self {
        let temp = (to - from).normalize_or_zero();
        let normal = Vec2::new(-temp.y, temp.x);

        Self { from, to, normal }
    }

    pub fn from(&self, position: Vec2) -> Vec2 {
        position + self.from
    }

    pub fn to(&self, position: Vec2) -> Vec2 {
        position + self.to
    }

    pub fn normal(&self) -> Vec2 {
        self.normal
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct WallQuery<'w> {
    pub wall: &'w Wall,
    pub transform: &'w Transform,
}

#[derive(Debug, Default, Component)]
pub struct WallDebug;

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

// TODO: hide
