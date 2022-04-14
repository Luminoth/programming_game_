use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

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
