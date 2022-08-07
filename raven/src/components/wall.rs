use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Component, Inspectable)]
pub struct Wall {
    pub from: Vec2,
    pub to: Vec2,

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
pub struct WallQuery {
    pub wall: &'static Wall,
    pub transform: &'static Transform,
}

#[derive(Debug, Default, Component)]
pub struct WallDebug;
