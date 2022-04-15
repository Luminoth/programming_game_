use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;

use super::state::StateMachine;

#[derive(Debug, Default, Component, Inspectable)]
pub struct SoccerTeam {
    pub team: Team,
}

pub type SoccerTeamStateMachine = StateMachine<SoccerTeamState>;

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

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ReceivingPlayer;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ClosestPlayer;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ControllingPlayer;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct SupportingPlayer;
