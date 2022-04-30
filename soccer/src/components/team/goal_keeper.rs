use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::actor::*;
use crate::components::agent::*;
use crate::components::state::impl_state_machine;
use crate::components::steering::*;
use crate::resources::*;

use super::*;

impl_state_machine!(
    GoalKeeper,
    TendGoal,
    ReturnHome,
    PutBallBackInPlay,
    InterceptBall
);

#[derive(Debug, Default, Component, Inspectable)]
pub struct GoalKeeper;

impl GoalKeeper {
    pub fn is_ball_within_keeper_range(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        ball_transform: &Transform,
    ) -> bool {
        let position = transform.translation.truncate();
        let ball_position = ball_transform.translation.truncate();
        position.distance_squared(ball_position) < params.keeper_in_ball_range_squared
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct GoalKeeperQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub player: &'w SoccerPlayer,
    pub goal_keeper: &'w GoalKeeper,
    pub team: &'w T,
    pub name: &'w Name,

    pub actor: &'w Actor,
    pub agent: &'w Agent,
    pub steering: &'w Steering,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct GoalKeeperQueryMut<'w, T>
where
    T: TeamColorMarker,
{
    pub player: &'w mut SoccerPlayer,
    pub goal_keeper: &'w GoalKeeper,
    pub team: &'w T,
    pub name: &'w Name,

    pub actor: &'w Actor,
    pub agent: &'w mut Agent,
    pub steering: &'w mut Steering,
    pub state_machine: &'w mut GoalKeeperStateMachine,
}
