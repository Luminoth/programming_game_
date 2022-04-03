#![allow(non_snake_case)]

use bevy::prelude::*;
use rand::Rng;

use crate::components::wife::{Wife, WifeStateMachine};
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

    pub fn enter(self, name: impl AsRef<str>, wife: &mut Wife) {
        match self {
            Self::DoHouseWork => (),
            Self::VisitBathroom => Self::VisitBathroom_enter(name),
            Self::CookStew => Self::CookStew_enter(name, wife),
        }
    }

    pub fn execute(
        self,
        entity: Entity,
        name: impl AsRef<str>,
        state_machine: &mut WifeStateMachine,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        match self {
            Self::DoHouseWork => Self::DoHouseWork_execute(name),
            Self::VisitBathroom => {
                Self::VisitBathroom_execute(entity, name, state_machine, exit_events, enter_events)
            }
            Self::CookStew => (),
        }
    }

    pub fn exit(self, name: impl AsRef<str>) {
        match self {
            Self::DoHouseWork => (),
            Self::VisitBathroom => Self::VisitBathroom_exit(name),
            Self::CookStew => (),
        }
    }
}

impl WifeState {
    fn DoHouseWork_execute(name: impl AsRef<str>) {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=2) {
            0 => info!("{}: Moppin' the floor", name.as_ref()),
            1 => info!("{}: Washin' the dishes", name.as_ref()),
            2 => info!("{}: Makin' the bed", name.as_ref()),
            _ => unreachable!(),
        }
    }
}

impl WifeState {
    fn VisitBathroom_enter(name: impl AsRef<str>) {
        info!(
            "{}: Walkin' to the can. Need to powda mah pretty li'lle nose",
            name.as_ref()
        );
    }

    fn VisitBathroom_execute(
        entity: Entity,
        name: impl AsRef<str>,
        state_machine: &mut WifeStateMachine,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        info!("{}: Ahhhhhh! Sweet relief!", name.as_ref());

        state_machine.revert_to_previous_state(entity, exit_events, enter_events);
    }

    fn VisitBathroom_exit(name: impl AsRef<str>) {
        info!("{}: Leavin' the Jon", name.as_ref());
    }
}

impl WifeState {
    fn CookStew_enter(name: impl AsRef<str>, wife: &mut Wife) {
        if wife.is_cooking() {
            return;
        }

        info!("{}: Puttin' the stew in the oven", name.as_ref());

        /*state_machine
        .message_dispatcher()
        .borrow()
        .defer_dispatch_message(entity.id(), entity.id(), Message::StewIsReady, 1.5);*/

        wife.start_cooking();
    }
}
