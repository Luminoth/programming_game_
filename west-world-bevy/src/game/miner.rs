#![allow(non_snake_case)]
#![allow(clippy::single_match)]

use bevy::prelude::*;
use chrono::prelude::*;

use crate::components::miner::*;
use crate::events::messaging::MessageEvent;
use crate::events::state::{StateEnterEvent, StateExitEvent};
use crate::resources::messaging::MessageDispatcher;

use super::state::State;
use super::Location;

pub const COMFORT_LEVEL: i64 = 5;
pub const MAX_NUGGETS: i64 = 3;
pub const THIRST_LEVEL: i64 = 5;
pub const TIREDNESS_THRESHOLD: i64 = 5;

pub type MinerStateEnterEvent = StateEnterEvent<MinerState>;
pub type MinerStateExitEvent = StateExitEvent<MinerState>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MinerState {
    EnterMineAndDigForNugget,
    VisitBankAndDepositGold,
    GoHomeAndSleepTilRested,
    QuenchThirst,
    EatStew,
}

impl Default for MinerState {
    fn default() -> Self {
        Self::GoHomeAndSleepTilRested
    }
}

impl State for MinerState {}

impl MinerState {
    pub fn execute_global() {}

    pub fn enter(
        self,
        entity: Entity,
        miner: MinerQueryItem,
        wife: &MinerWife,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        match self {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_enter(miner),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_enter(miner),
            Self::GoHomeAndSleepTilRested => {
                Self::GoHomeAndSleepTilRested_enter(entity, miner, wife, message_dispatcher)
            }
            Self::QuenchThirst => Self::QuenchThirst_enter(miner),
            Self::EatStew => Self::EatStew_enter(miner),
        }
    }

    pub fn execute(
        self,
        entity: Entity,
        miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        match self {
            Self::EnterMineAndDigForNugget => {
                Self::EnterMineAndDigForNugget_execute(entity, miner, exit_events, enter_events)
            }
            Self::VisitBankAndDepositGold => {
                Self::VisitBankAndDepositGold_execute(entity, miner, exit_events, enter_events)
            }
            Self::GoHomeAndSleepTilRested => {
                Self::GoHomeAndSleepTilRested_execute(entity, miner, exit_events, enter_events)
            }
            Self::QuenchThirst => {
                Self::QuenchThirst_execute(entity, miner, exit_events, enter_events)
            }
            Self::EatStew => Self::EatStew_execute(entity, miner, exit_events, enter_events),
        }
    }

    pub fn exit(self, miner: MinerQueryItem) {
        match self {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_exit(miner),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_exit(miner),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_exit(miner),
            Self::QuenchThirst => Self::QuenchThirst_exit(miner),
            Self::EatStew => Self::EatStew_exit(miner),
        }
    }

    pub fn on_message(
        self,
        message: &MessageEvent,
        entity: Entity,
        miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        match self {
            Self::EnterMineAndDigForNugget => (),
            Self::VisitBankAndDepositGold => (),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_on_message(
                message,
                entity,
                miner,
                exit_events,
                enter_events,
            ),
            Self::QuenchThirst => (),
            Self::EatStew => (),
        }
    }
}

impl MinerState {
    fn EnterMineAndDigForNugget_enter(mut miner: MinerQueryItem) {
        if miner.miner.location != Location::GoldMine {
            info!("{}: Walkin' to the gold mine", miner.name.as_ref());

            miner.miner.location = Location::GoldMine;
        }
    }

    fn EnterMineAndDigForNugget_execute(
        entity: Entity,
        mut miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        miner.stats.mine_gold();

        info!("{}: Pickin' up a nugget", miner.name.as_ref());

        if miner.stats.are_pockets_full() {
            miner.state_machine.change_state(
                entity,
                Self::VisitBankAndDepositGold,
                exit_events,
                enter_events,
            );
        } else if miner.stats.is_thirsty() {
            miner
                .state_machine
                .change_state(entity, Self::QuenchThirst, exit_events, enter_events);
        }
    }

    fn EnterMineAndDigForNugget_exit(miner: MinerQueryItem) {
        info!(
            "{}: Ah'm leavin' the gold mine with mah pockets full o' sweet gold",
            miner.name.as_ref()
        )
    }
}

impl MinerState {
    fn VisitBankAndDepositGold_enter(mut miner: MinerQueryItem) {
        if miner.miner.location != Location::Bank {
            info!("{}: Goin' to the bank. Yes siree", miner.name.as_ref());

            miner.miner.location = Location::Bank;
        }
    }

    fn VisitBankAndDepositGold_execute(
        entity: Entity,
        mut miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        miner.stats.transfer_gold_to_wealth();

        info!(
            "{}: Depositing gold. Total savings now: {}",
            miner.name.as_ref(),
            miner.stats.wealth()
        );

        if miner.stats.wealth() >= COMFORT_LEVEL {
            info!(
                "{}: WooHoo! Rich enough for now. Back home to mah li'lle lady",
                miner.name.as_ref()
            );

            miner.state_machine.change_state(
                entity,
                Self::GoHomeAndSleepTilRested,
                exit_events,
                enter_events,
            );
        } else {
            miner.state_machine.change_state(
                entity,
                Self::EnterMineAndDigForNugget,
                exit_events,
                enter_events,
            );
        }
    }

    fn VisitBankAndDepositGold_exit(miner: MinerQueryItem) {
        info!("{}: Leavin' the bank", miner.name.as_ref());
    }
}

impl MinerState {
    fn GoHomeAndSleepTilRested_enter(
        entity: Entity,
        mut miner: MinerQueryItem,
        wife: &MinerWife,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        if miner.miner.location != Location::Shack {
            info!("{}: Walkin' home", miner.name.as_ref());

            miner.miner.location = Location::Shack;

            message_dispatcher.dispatch_message(wife.wife_id, MessageEvent::HiHoneyImHome(entity));
        }
    }

    fn GoHomeAndSleepTilRested_execute(
        entity: Entity,
        mut miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        if !miner.stats.is_fatigued() {
            info!(
                "{}: What a God darn fantastic nap! Time to find more gold",
                miner.name.as_ref()
            );

            miner.state_machine.change_state(
                entity,
                Self::EnterMineAndDigForNugget,
                exit_events,
                enter_events,
            );
        } else {
            miner.stats.rest();

            info!("{}: ZZZZ... ", miner.name.as_ref());

            miner.state_machine.change_state(
                entity,
                Self::GoHomeAndSleepTilRested,
                exit_events,
                enter_events,
            );
        }
    }

    fn GoHomeAndSleepTilRested_exit(miner: MinerQueryItem) {
        info!("{}: Leaving the house", miner.name.as_ref());
    }

    fn GoHomeAndSleepTilRested_on_message(
        message: &MessageEvent,
        entity: Entity,
        mut miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        match message {
            MessageEvent::StewIsReady(_) => {
                let now = Utc::now();

                debug!(
                    "Message handled by {} at time: {}",
                    miner.name.as_ref(),
                    now
                );
                info!("{}: Ok hun, ahm a-comin'!", miner.name.as_ref());

                miner
                    .state_machine
                    .change_state(entity, Self::EatStew, exit_events, enter_events);
            }
            _ => (),
        }
    }
}

impl MinerState {
    fn QuenchThirst_enter(mut miner: MinerQueryItem) {
        if miner.miner.location != Location::Saloon {
            info!(
                "{}: Boy, ah sure is thusty! Walking to the saloon",
                miner.name.as_ref()
            );

            miner.miner.location = Location::Shack;
        }
    }

    fn QuenchThirst_execute(
        entity: Entity,
        mut miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        if miner.stats.is_thirsty() {
            miner.stats.buy_and_drink_a_whiskey();

            info!("{}: That's mighty fine sippin liquer", miner.name.as_ref());

            miner.state_machine.change_state(
                entity,
                Self::EnterMineAndDigForNugget,
                exit_events,
                enter_events,
            );
        } else {
            unreachable!();
        }
    }

    fn QuenchThirst_exit(miner: MinerQueryItem) {
        info!("{}: Leaving the saloon, feelin' good", miner.name.as_ref());
    }
}

impl MinerState {
    fn EatStew_enter(miner: MinerQueryItem) {
        info!("{}: Smells reaaal good Elsa!", miner.name.as_ref());
    }

    fn EatStew_execute(
        entity: Entity,
        mut miner: MinerQueryItem,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        info!("{}: Tastes reaaal good too!", miner.name.as_ref());

        miner
            .state_machine
            .revert_to_previous_state(entity, exit_events, enter_events);
    }

    fn EatStew_exit(miner: MinerQueryItem) {
        info!(
            "{}: Thankya li'lle lady. Ah better get back to whatever ah wuz doin'",
            miner.name.as_ref()
        );
    }
}
