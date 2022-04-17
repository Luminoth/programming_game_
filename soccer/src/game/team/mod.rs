#![allow(non_snake_case)]

mod field_player;
mod goalie;

pub use field_player::*;
pub use goalie::*;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::components::team::*;
use crate::events::messaging::MessageEvent;
use crate::resources::messaging::MessageDispatcher;

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

    #[allow(dead_code)]
    pub fn sign(&self) -> f32 {
        match self {
            Self::Red => -1.0,
            Self::Blue => 1.0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Inspectable)]
pub enum SoccerTeamState {
    PrepareForKickOff,
    Defending,
    Attacking,
}

impl Default for SoccerTeamState {
    fn default() -> Self {
        Self::PrepareForKickOff
    }
}

impl State for SoccerTeamState {}

impl SoccerTeamState {
    pub fn execute_global(soccer_team: SoccerTeamQueryMutItem) {
        debug!(
            "executing global state for team {:?}",
            soccer_team.team.team
        );
    }

    pub fn enter(
        self,
        commands: &mut Commands,
        message_dispatcher: &mut MessageDispatcher,
        team: &SoccerTeam,
        receiving: Option<Entity>,
        closest: Option<Entity>,
        controlling: Option<Entity>,
        supporting: Option<Entity>,
    ) {
        match self {
            Self::PrepareForKickOff => Self::PrepareForKickOff_enter(
                commands,
                message_dispatcher,
                team,
                receiving,
                closest,
                controlling,
                supporting,
            ),
            Self::Defending => (),
            Self::Attacking => (),
        }
    }
}

impl SoccerTeamState {
    fn PrepareForKickOff_enter(
        commands: &mut Commands,
        message_dispatcher: &mut MessageDispatcher,
        team: &SoccerTeam,
        receiving: Option<Entity>,
        closest: Option<Entity>,
        controlling: Option<Entity>,
        supporting: Option<Entity>,
    ) {
        // reset player positions

        if let Some(receiving) = receiving {
            commands.entity(receiving).remove::<ReceivingPlayer>();
        }

        if let Some(closest) = closest {
            commands.entity(closest).remove::<ClosestPlayer>();
        }

        if let Some(controlling) = controlling {
            commands.entity(controlling).remove::<ControllingPlayer>();
        }

        if let Some(supporting) = supporting {
            commands.entity(supporting).remove::<SupportingPlayer>();
        }

        // send players home
        message_dispatcher.dispatch_message(None, MessageEvent::GoHome(team.team));
    }
}
