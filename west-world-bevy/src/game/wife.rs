#![allow(non_snake_case)]

use bevy::prelude::*;
use rand::Rng;

use crate::components::state::StateMachine;

use super::state::State;

const BATHROOM_CHANCE: f32 = 0.1;

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

impl State for WifeState {
    fn execute_global(state_machine: &mut StateMachine<WifeState>) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < BATHROOM_CHANCE {
            state_machine.change_state(Self::VisitBathroom);
        }
    }

    fn enter(self, state_machine: &mut StateMachine<WifeState>) {}

    fn execute(self, state_machine: &mut StateMachine<WifeState>) {
        match self {
            WifeState::DoHouseWork => (),
            WifeState::VisitBathroom => (),
            WifeState::CookStew => (),
        }
    }

    fn exit(self, state_machine: &mut StateMachine<WifeState>) {}
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
