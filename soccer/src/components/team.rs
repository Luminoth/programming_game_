use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;
use crate::game::Team;

use super::state::StateMachine;

#[derive(Debug, Default, Component, Inspectable)]
pub struct FieldPlayer {
    pub team: Team,
}

pub type FieldPlayerStateMachine = StateMachine<FieldPlayerState>;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goalie {
    pub team: Team,
}

pub type GoalieStateMachine = StateMachine<GoalieState>;
