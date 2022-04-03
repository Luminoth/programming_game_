#![allow(non_snake_case)]

use bevy::prelude::*;
use chrono::prelude::*;
use rand::Rng;

use crate::components::wife::*;
use crate::events::state::{StateEnterEvent, StateExitEvent};
use crate::resources::messaging::MessageDispatcher;

use super::messaging::Message;
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
        debug!("executing wife global state for {}", name.as_ref());

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < BATHROOM_CHANCE {
            state_machine.change_state(entity, Self::VisitBathroom, exit_events, enter_events);
        }
    }

    pub fn on_message_global(
        message: &Message,
        entity: Entity,
        name: impl AsRef<str>,
        state_machine: &mut WifeStateMachine,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        match message {
            Message::HiHoneyImHome => {
                let now = Utc::now();

                debug!("Message handled by {} at time: {}", name.as_ref(), now);
                info!(
                    "{}: Hi honey. Let me make you some of mah fine country stew",
                    name.as_ref()
                );

                state_machine.change_state(entity, Self::CookStew, exit_events, enter_events);
            }
            _ => (),
        }
    }

    pub fn enter(
        self,
        entity: Entity,
        name: impl AsRef<str>,
        wife: &mut Wife,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        match self {
            Self::DoHouseWork => (),
            Self::VisitBathroom => Self::VisitBathroom_enter(name),
            Self::CookStew => Self::CookStew_enter(entity, name, wife, message_dispatcher),
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

    pub fn on_message(
        self,
        message: &Message,
        entity: Entity,
        name: impl AsRef<str>,
        wife: &mut Wife,
        miner: &WifeMiner,
        state_machine: &mut WifeStateMachine,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        match self {
            Self::DoHouseWork => (),
            Self::VisitBathroom => (),
            Self::CookStew => Self::CookStew_on_message(
                message,
                entity,
                name,
                wife,
                miner,
                state_machine,
                exit_events,
                enter_events,
                message_dispatcher,
            ),
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
    fn CookStew_enter(
        entity: Entity,
        name: impl AsRef<str>,
        wife: &mut Wife,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        if wife.cooking {
            return;
        }

        info!("{}: Puttin' the stew in the oven", name.as_ref());

        message_dispatcher.defer_dispatch_message(entity, entity, Message::StewIsReady, 1.5);

        wife.cooking = true;
    }

    fn CookStew_on_message(
        message: &Message,
        entity: Entity,
        name: impl AsRef<str>,
        wife: &mut Wife,
        miner: &WifeMiner,
        state_machine: &mut WifeStateMachine,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        match message {
            Message::StewIsReady => {
                let now = Utc::now();

                debug!("Message received by {} at time: {}", name.as_ref(), now);
                info!("{}: Stew ready! Let's eat", name.as_ref());

                message_dispatcher.dispatch_message(entity, miner.miner_id, Message::StewIsReady);

                wife.cooking = false;

                state_machine.change_state(entity, Self::DoHouseWork, exit_events, enter_events);
            }
            _ => (),
        }
    }
}
