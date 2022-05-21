use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// rename of the book's BaseGameEntity
#[derive(Debug, Default, Component, Inspectable)]
pub struct Actor;

// this doesn't include a transform because
// most of the time the PhysicalQuery captures that
#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct ActorQuery<'w> {
    pub actor: &'w Actor,
    pub name: &'w Name,
}
