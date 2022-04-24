use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;

use super::super::state::impl_state_machine;

impl_state_machine!(FieldPlayer, Idle);

#[derive(Debug, Default, Component, Inspectable)]
pub struct FieldPlayer {
    pub team: Team,
    pub ready: bool,
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
