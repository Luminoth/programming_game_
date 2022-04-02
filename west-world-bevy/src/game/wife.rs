#![allow(non_snake_case)]

use bevy::prelude::*;
use rand::Rng;

use crate::components::wife::WifeStateMachine;
use crate::events::state::{StateEnterEvent, StateExitEvent};

use super::state::State;

pub const BATHROOM_CHANCE: f32 = 0.1;

pub type WifeStateEnterEvent = StateEnterEvent<WifeState>;
pub type WifeStateExitEvent = StateExitEvent<WifeState>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WifeState {
    DoHouseWork,
    VisitBathroom,
    CookStew,
}

impl Default for WifeState {
    fn default() -> Self {
        Self::DoHouseWork
    }
}

impl State for WifeState {}

impl WifeState {
    pub fn execute_global(
        entity: Entity,
        name: impl AsRef<str>,
        state_machine: &mut WifeStateMachine,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        info!("executing wife global state for {}", name.as_ref());

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < BATHROOM_CHANCE {
            state_machine.change_state(entity, Self::VisitBathroom, exit_events, enter_events);
        }
    }

    pub fn enter(self, state_machine: &mut WifeStateMachine) {}

    pub fn execute(self, state_machine: &mut WifeStateMachine) {
        match self {
            WifeState::DoHouseWork => (),
            WifeState::VisitBathroom => (),
            WifeState::CookStew => (),
        }
    }

    pub fn exit(self, state_machine: &mut WifeStateMachine) {}
}

impl WifeState {
    fn DoHouseWork_execute(name: &Name) {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=2) {
            0 => info!("{}: Moppin' the floor", name.as_str()),
            1 => info!("{}: Washin' the dishes", name.as_str()),
            2 => info!("{}: Makin' the bed", name.as_str()),
            _ => unreachable!(),
        }
    }
}
