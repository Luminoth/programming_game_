use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::Team;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goal {
    pub team: Team,
    pub facing: Vec2,

    // scoring offsets
    pub top: Vec2,
    pub bottom: Vec2,
    pub score_center: Vec2,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct GoalDebug;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct GoalQuery<'w> {
    pub goal: &'w Goal,
    pub transform: &'w Transform,
}
