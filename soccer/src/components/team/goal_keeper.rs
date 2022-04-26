use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::steering::*;
use crate::game::team::*;
use crate::resources::pitch::*;

use super::super::state::impl_state_machine;

impl_state_machine!(GoalKeeper, TendGoal);

#[derive(Debug, Default, Component, Inspectable)]
pub struct GoalKeeper {
    pub team: Team,
    pub home_region: usize,
    pub default_region: usize,
}

impl GoalKeeper {
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
pub struct GoalKeeperQuery<'w> {
    pub goal_keeper: &'w GoalKeeper,
    pub steering: &'w Steering,
    pub name: &'w Name,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct GGoalKeeperQueryMut<'w> {
    pub goal_keeper: &'w mut GoalKeeper,
    pub steering: &'w mut Steering,
    pub state_machine: &'w mut GoalKeeperStateMachine,
    pub name: &'w Name,
}
