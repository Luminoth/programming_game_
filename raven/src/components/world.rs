use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Wall;

#[derive(Debug, Default, Component, Inspectable)]
pub struct SpawnPoint {
    pub offset: Vec2,
}

impl SpawnPoint {
    pub fn get_spawn_position(&self, transform: &Transform) -> Vec2 {
        transform.translation.truncate() + self.offset
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct SpawnPointQuery<'w> {
    pub spawnpoint: &'w SpawnPoint,
    pub transform: &'w Transform,
}
