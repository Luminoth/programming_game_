use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;

use super::super::state::StateMachine;

pub type GoalieStateMachine = StateMachine<GoalieState>;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goalie {
    pub team: Team,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct GoalieQueryMut<'w> {
    pub goalie: &'w mut Goalie,
    pub state_machine: &'w mut GoalieStateMachine,
    pub name: &'w Name,
}
