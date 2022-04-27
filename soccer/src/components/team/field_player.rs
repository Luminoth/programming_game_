use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::agent::*;
use crate::components::steering::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::*;

use super::super::state::impl_state_machine;

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
pub struct FieldPlayer {
    pub team: Team,
    pub home_region: usize,
    pub default_region: usize,
}

impl FieldPlayer {
    pub fn is_in_home_region(&self, transform: &Transform, pitch: &Pitch) -> bool {
        pitch
            .regions
            .get(self.home_region)
            .unwrap()
            .is_inside_half(transform.translation.truncate())
    }

    pub fn is_ball_within_receiving_range(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        ball_transform: &Transform,
    ) -> bool {
        let position = transform.translation.truncate();
        let ball_position = ball_transform.translation.truncate();
        position.distance_squared(ball_position) < params.ball_within_receiving_range_squared
    }

    pub fn is_ball_within_kicking_range(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        ball_transform: &Transform,
    ) -> bool {
        let position = transform.translation.truncate();
        let ball_position = ball_transform.translation.truncate();
        position.distance_squared(ball_position) < params.player_kicking_distance_squared
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct FieldPlayerQuery<'w> {
    pub player: &'w FieldPlayer,
    pub agent: &'w Agent,
    pub steering: &'w Steering,
    pub name: &'w Name,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct FieldPlayerQueryMut<'w> {
    pub player: &'w mut FieldPlayer,
    pub agent: &'w mut Agent,
    pub steering: &'w mut Steering,
    pub state_machine: &'w mut FieldPlayerStateMachine,
    pub name: &'w Name,
}
