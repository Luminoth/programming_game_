use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;
use crate::resources::pitch::*;

use super::super::state::impl_state_machine;

impl_state_machine!(FieldPlayer, Idle);

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
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct FieldPlayerQuery<'w> {
    pub player: &'w FieldPlayer,
    pub name: &'w Name,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct FieldPlayerQueryMut<'w> {
    pub player: &'w mut FieldPlayer,
    pub state_machine: &'w mut FieldPlayerStateMachine,
    pub name: &'w Name,
}
