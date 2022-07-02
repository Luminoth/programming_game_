use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct Obstacle;

#[derive(Debug, Default, Component)]
pub struct ObstacleDebug;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Wall {
    pub extents: Vec2,
    pub facing: Vec2,
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct WallQuery<'w> {
    pub wall: &'w Wall,
    pub transform: &'w Transform,
}
