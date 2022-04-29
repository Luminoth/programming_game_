use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::team::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goal {
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
pub struct GoalQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub goal: &'w Goal,
    pub team: &'w T,

    pub transform: &'w Transform,
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct AnyTeamGoalQuery<'w> {
    pub goal: &'w Goal,
    pub blue_team: Option<&'w BlueTeam>,
    pub red_team: Option<&'w RedTeam>,

    pub transform: &'w Transform,
}
