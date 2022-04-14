use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::components::team::*;

use super::state::State;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub enum Team {
    Red,
    Blue,
}

impl Default for Team {
    fn default() -> Self {
        Self::Red
    }
}

impl Team {
    pub fn color(&self) -> Color {
        match self {
            Self::Red => Color::RED,
            Self::Blue => Color::BLUE,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Inspectable)]
pub enum SoccerTeamState {
    Idle,
}

impl Default for SoccerTeamState {
    fn default() -> Self {
        Self::Idle
    }
}

impl State for SoccerTeamState {}

impl SoccerTeamState {
    pub fn execute_global(soccer_team: &SoccerTeam) {
        debug!("executing global state for team {:?}", soccer_team.team);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Inspectable)]
pub enum FieldPlayerState {
    Idle,
}

impl Default for FieldPlayerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl State for FieldPlayerState {}

impl FieldPlayerState {
    pub fn execute_global(name: impl AsRef<str>) {
        debug!("executing global state for player {}", name.as_ref());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Inspectable)]
pub enum GoalieState {
    Idle,
}

impl Default for GoalieState {
    fn default() -> Self {
        Self::Idle
    }
}

impl State for GoalieState {}

impl GoalieState {
    pub fn execute_global(name: impl AsRef<str>) {
        debug!("executing global state for goalie {}", name.as_ref());
    }
}
