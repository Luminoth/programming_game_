use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::actor::*;
use crate::components::agent::*;
use crate::components::state::impl_state_machine;
use crate::components::steering::*;
use crate::game::Cooldown;
use crate::resources::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub enum FieldPlayerRole {
    Attacker,
    Defender,
}

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

#[derive(Debug, Component, Inspectable)]
pub struct FieldPlayer {
    pub role: FieldPlayerRole,

    #[inspectable(ignore)]
    pub kick_cooldown: Cooldown,
}

impl FieldPlayer {
    pub fn new(params: &SimulationParams, role: FieldPlayerRole) -> Self {
        Self {
            role,
            kick_cooldown: Cooldown::from_seconds(1.0 / params.player_kick_frequency as f32),
        }
    }

    pub fn is_ready_for_next_kick(&self) -> bool {
        self.kick_cooldown.available()
    }

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

    pub fn is_ahead_of_attacker(
        &self,
        transform: &Transform,
        controller_transform: &Transform,
        opponent_goal: (&Goal, &Transform),
    ) -> bool {
        let position = transform.translation.truncate();
        let controller_position = controller_transform.translation.truncate();
        let opponent_goal_center = opponent_goal.0.get_score_center(opponent_goal.1);

        (position.x - opponent_goal_center.x).abs()
            < (controller_position.x - opponent_goal_center.x).abs()
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

    pub actor: &'w Actor,
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
    pub field_player: &'w mut FieldPlayer,
    pub team: &'w T,
    pub name: &'w Name,

    pub actor: &'w Actor,
    pub agent: &'w mut Agent,
    pub steering: &'w mut Steering,
    pub state_machine: &'w mut FieldPlayerStateMachine,
}
