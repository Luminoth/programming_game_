#![allow(non_snake_case)]
#![allow(clippy::single_match)]

use bevy::prelude::*;
use chrono::prelude::*;
use rand::Rng;

use crate::components::wife::*;
use crate::events::messaging::MessageEvent;
use crate::events::state::{StateEnterEvent, StateExitEvent};
use crate::resources::messaging::MessageDispatcher;

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
        mut wife: WifeQueryItem,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        debug!("executing wife global state for {}", wife.name.as_ref());

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < BATHROOM_CHANCE {
            wife.state_machine
                .change_state(entity, Self::VisitBathroom, exit_events, enter_events);
        }
    }

    pub fn on_message_global(
        message: &MessageEvent,
        entity: Entity,
        mut wife: WifeQueryItem,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        match message {
            MessageEvent::HiHoneyImHome(_) => {
                let now = Utc::now();

                debug!("Message handled by {} at time: {}", wife.name.as_ref(), now);
                info!(
                    "{}: Hi honey. Let me make you some of mah fine country stew",
                    wife.name.as_ref()
                );

                wife.state_machine
                    .change_state(entity, Self::CookStew, exit_events, enter_events);
            }
            _ => (),
        }
    }

    pub fn enter(
        self,
        entity: Entity,
        wife: WifeQueryItem,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        match self {
            Self::DoHouseWork => (),
            Self::VisitBathroom => Self::VisitBathroom_enter(wife),
            Self::CookStew => Self::CookStew_enter(entity, wife, message_dispatcher),
        }
    }

    pub fn execute(
        self,
        entity: Entity,
        wife: WifeQueryItem,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        match self {
            Self::DoHouseWork => Self::DoHouseWork_execute(wife),
            Self::VisitBathroom => {
                Self::VisitBathroom_execute(entity, wife, exit_events, enter_events)
            }
            Self::CookStew => (),
        }
    }

    pub fn exit(self, wife: WifeQueryItem) {
        match self {
            Self::DoHouseWork => (),
            Self::VisitBathroom => Self::VisitBathroom_exit(wife),
            Self::CookStew => (),
        }
    }

    pub fn on_message(
        self,
        message: &MessageEvent,
        entity: Entity,
        wife: WifeQueryItem,
        miner: &WifeMiner,
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
                wife,
                miner,
                exit_events,
                enter_events,
                message_dispatcher,
            ),
        }
    }
}

impl WifeState {
    fn DoHouseWork_execute(wife: WifeQueryItem) {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=2) {
            0 => info!("{}: Moppin' the floor", wife.name.as_ref()),
            1 => info!("{}: Washin' the dishes", wife.name.as_ref()),
            2 => info!("{}: Makin' the bed", wife.name.as_ref()),
            _ => unreachable!(),
        }
    }
}

impl WifeState {
    fn VisitBathroom_enter(wife: WifeQueryItem) {
        info!(
            "{}: Walkin' to the can. Need to powda mah pretty li'lle nose",
            wife.name.as_ref()
        );
    }

    fn VisitBathroom_execute(
        entity: Entity,
        mut wife: WifeQueryItem,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
    ) {
        info!("{}: Ahhhhhh! Sweet relief!", wife.name.as_ref());

        wife.state_machine
            .revert_to_previous_state(entity, exit_events, enter_events);
    }

    fn VisitBathroom_exit(wife: WifeQueryItem) {
        info!("{}: Leavin' the Jon", wife.name.as_ref());
    }
}

impl WifeState {
    fn CookStew_enter(
        entity: Entity,
        mut wife: WifeQueryItem,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        if wife.wife.cooking {
            return;
        }

        info!("{}: Puttin' the stew in the oven", wife.name.as_ref());

        message_dispatcher.defer_dispatch_message(entity, MessageEvent::StewIsReady(entity), 1.5);

        wife.wife.cooking = true;
    }

    fn CookStew_on_message(
        message: &MessageEvent,
        entity: Entity,
        mut wife: WifeQueryItem,
        miner: &WifeMiner,
        exit_events: &mut EventWriter<WifeStateExitEvent>,
        enter_events: &mut EventWriter<WifeStateEnterEvent>,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        match message {
            MessageEvent::StewIsReady(_) => {
                let now = Utc::now();

                debug!(
                    "Message received by {} at time: {}",
                    wife.name.as_ref(),
                    now
                );
                info!("{}: Stew ready! Let's eat", wife.name.as_ref());

                message_dispatcher
                    .dispatch_message(miner.miner_id, MessageEvent::StewIsReady(entity));

                wife.wife.cooking = false;

                wife.state_machine.change_state(
                    entity,
                    Self::DoHouseWork,
                    exit_events,
                    enter_events,
                );
            }
            _ => (),
        }
    }
}
