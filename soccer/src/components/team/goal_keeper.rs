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

    pub fn is_ball_within_range_for_intercept<T>(
        &self,
        params: &SimulationParams,
        goal: &TeamGoalQueryItem<T>,
        ball_transform: &Transform,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        let ball_position = ball_transform.translation.truncate();
        goal.goal
            .get_score_center(goal.transform)
            .distance_squared(ball_position)
            <= params.goal_keeper_intercept_range_squared
    }

    pub fn is_too_far_from_goal_mouth<T>(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        goal: &TeamGoalQueryItem<T>,
        ball_transform: &Transform,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        let position = transform.translation.truncate();
        position.distance_squared(self.get_rear_interpose_target(params, goal, ball_transform))
            > params.goal_keeper_intercept_range_squared
    }

    pub fn get_rear_interpose_target<T>(
        &self,
        params: &SimulationParams,
        goal: &TeamGoalQueryItem<T>,
        ball_transform: &Transform,
    ) -> Vec2
    where
        T: TeamColorMarker,
    {
        let ball_position = ball_transform.translation.truncate();

        let target_x = goal.goal.get_score_center(goal.transform).x;
        let target_y = params.goal_extents.y * 0.5
            + (ball_position.y * params.goal_extents.x) / params.pitch_extents.y;

        Vec2::new(target_x, target_y)
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
