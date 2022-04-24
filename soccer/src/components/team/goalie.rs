use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;

use super::super::state::impl_state_machine;

impl_state_machine!(Goalie, Idle);

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goalie {
    pub team: Team,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct GoalieQuery<'w> {
    pub goalie: &'w Goalie,
    pub name: &'w Name,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct GoalieQueryMut<'w> {
    pub goalie: &'w mut Goalie,
    pub state_machine: &'w mut GoalieStateMachine,
    pub name: &'w Name,
}
