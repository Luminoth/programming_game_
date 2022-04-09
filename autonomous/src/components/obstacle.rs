use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Obstacle;

#[derive(Debug, Default, Component)]
pub struct ObstacleDebug;

#[derive(Debug, Default, Component)]
pub struct ObstacleAvoidance {
    pub box_length: f32,
}

#[derive(Debug, Default, Component)]
pub struct ObstacleAvoidanceDebug;

// TODO: walls for wall avoidance

// TODO: hide
