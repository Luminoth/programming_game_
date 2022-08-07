use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct MainCamera;

#[derive(Debug, Default, Component)]
pub struct UiCamera;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct CameraQuery {
    pub camera: &'static Camera,
    pub transform: &'static Transform,
}
