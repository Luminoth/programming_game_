use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;
use crate::resources::pitch::*;

use super::super::state::impl_state_machine;

impl_state_machine!(Goalie, Idle);

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goalie {
    pub team: Team,
    pub home_region: usize,
    pub default_region: usize,
}

impl Goalie {
    pub fn is_in_home_region(&self, transform: &Transform, pitch: &Pitch) -> bool {
        pitch
            .regions
            .get(self.home_region)
            .unwrap()
            .is_inside(transform.translation.truncate())
    }
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
