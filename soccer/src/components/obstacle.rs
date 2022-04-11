use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Obstacle;

#[derive(Debug, Default, Component)]
pub struct ObstacleDebug;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Wall;

#[derive(Debug, Default, Component)]
pub struct WallDebug;
