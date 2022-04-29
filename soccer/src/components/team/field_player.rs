use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::agent::*;
use crate::components::goal::*;
use crate::components::state::impl_state_machine;
use crate::components::steering::*;
use crate::resources::*;

use super::*;

impl_state_machine!(
    FieldPlayer,
    Wait,
    ReceiveBall,
    KickBall,
    Dribble,
    ChaseBall,
    ReturnToHomeRegion,
    SupportAttacker
);

#[derive(Debug, Default, Component, Inspectable)]
pub struct FieldPlayer;

impl FieldPlayer {
    pub fn is_ball_within_receiving_range(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        ball_position: Vec2,
    ) -> bool {
        let position = transform.translation.truncate();
        position.distance_squared(ball_position) < params.ball_within_receiving_range_squared
    }

    pub fn is_ball_within_kicking_range(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        ball_position: Vec2,
    ) -> bool {
        let position = transform.translation.truncate();
        position.distance_squared(ball_position) < params.player_kicking_distance_squared
    }

    pub fn is_ahead_of_attacker<T>(
        &self,
        team: &T,
        transform: &Transform,
        controller_transform: &Transform,
        goals: &Query<AnyTeamGoalQuery>,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        let position = transform.translation.truncate();
        let controller_position = controller_transform.translation.truncate();
        let opponent_goal_position = Goal::get_opponent_goal_position(team, goals).unwrap();

        (position.x - opponent_goal_position.x).abs()
            < (controller_position.x - opponent_goal_position.x).abs()
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct FieldPlayerQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub player: &'w SoccerPlayer,
    pub field_player: &'w FieldPlayer,
    pub team: &'w T,
    pub name: &'w Name,

    pub agent: &'w Agent,
    pub steering: &'w Steering,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct FieldPlayerQueryMut<'w, T>
where
    T: TeamColorMarker,
{
    pub player: &'w mut SoccerPlayer,
    pub field_player: &'w FieldPlayer,
    pub team: &'w T,
    pub name: &'w Name,

    pub agent: &'w mut Agent,
    pub steering: &'w mut Steering,
    pub state_machine: &'w mut FieldPlayerStateMachine,
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct AnyTeamFieldPlayerQuery<'w> {
    pub player: &'w SoccerPlayer,
    pub field_player: &'w FieldPlayer,
    pub blue_team: Option<&'w BlueTeam>,
    pub red_team: Option<&'w RedTeam>,
    pub name: &'w Name,

    pub agent: &'w Agent,
    pub steering: &'w Steering,
}
