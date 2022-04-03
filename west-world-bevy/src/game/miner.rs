#![allow(non_snake_case)]

use bevy::prelude::*;
use chrono::prelude::*;

use crate::components::miner::*;
use crate::events::state::{StateEnterEvent, StateExitEvent};
use crate::resources::messaging::MessageDispatcher;

use super::messaging::Message;
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
        name: impl AsRef<str>,
        miner: &mut Miner,
        wife: &MinerWife,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        match self {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_enter(name, miner),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_enter(name, miner),
            Self::GoHomeAndSleepTilRested => {
                Self::GoHomeAndSleepTilRested_enter(entity, name, miner, wife, message_dispatcher)
            }
            Self::QuenchThirst => Self::QuenchThirst_enter(name, miner),
            Self::EatStew => Self::EatStew_enter(name),
        }
    }

    pub fn execute(
        self,
        entity: Entity,
        name: impl AsRef<str>,
        stats: &mut Stats,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        match self {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_execute(
                entity,
                name,
                stats,
                state_machine,
                exit_events,
                enter_events,
            ),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_execute(
                entity,
                name,
                stats,
                state_machine,
                exit_events,
                enter_events,
            ),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_execute(
                entity,
                name,
                stats,
                state_machine,
                exit_events,
                enter_events,
            ),
            Self::QuenchThirst => Self::QuenchThirst_execute(
                entity,
                name,
                stats,
                state_machine,
                exit_events,
                enter_events,
            ),
            Self::EatStew => {
                Self::EatStew_execute(entity, name, state_machine, exit_events, enter_events)
            }
        }
    }

    pub fn exit(self, name: impl AsRef<str>) {
        match self {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_exit(name),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_exit(name),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_exit(name),
            Self::QuenchThirst => Self::QuenchThirst_exit(name),
            Self::EatStew => Self::EatStew_exit(name),
        }
    }

    pub fn on_message(
        self,
        message: &Message,
        entity: Entity,
        name: impl AsRef<str>,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        match self {
            Self::EnterMineAndDigForNugget => (),
            Self::VisitBankAndDepositGold => (),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_on_message(
                message,
                entity,
                name,
                state_machine,
                exit_events,
                enter_events,
            ),
            Self::QuenchThirst => (),
            Self::EatStew => (),
        }
    }
}

impl MinerState {
    fn EnterMineAndDigForNugget_enter(name: impl AsRef<str>, miner: &mut Miner) {
        if miner.location != Location::GoldMine {
            info!("{}: Walkin' to the gold mine", name.as_ref());

            miner.location = Location::GoldMine;
        }
    }

    fn EnterMineAndDigForNugget_execute(
        entity: Entity,
        name: impl AsRef<str>,
        stats: &mut Stats,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        stats.mine_gold();

        info!("{}: Pickin' up a nugget", name.as_ref());

        if stats.are_pockets_full() {
            state_machine.change_state(
                entity,
                Self::VisitBankAndDepositGold,
                exit_events,
                enter_events,
            );
        } else if stats.is_thirsty() {
            state_machine.change_state(entity, Self::QuenchThirst, exit_events, enter_events);
        }
    }

    fn EnterMineAndDigForNugget_exit(name: impl AsRef<str>) {
        info!(
            "{}: Ah'm leavin' the gold mine with mah pockets full o' sweet gold",
            name.as_ref()
        )
    }
}

impl MinerState {
    fn VisitBankAndDepositGold_enter(name: impl AsRef<str>, miner: &mut Miner) {
        if miner.location != Location::Bank {
            info!("{}: Goin' to the bank. Yes siree", name.as_ref());

            miner.location = Location::Bank;
        }
    }

    fn VisitBankAndDepositGold_execute(
        entity: Entity,
        name: impl AsRef<str>,
        stats: &mut Stats,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        stats.transfer_gold_to_wealth();

        info!(
            "{}: Depositing gold. Total savings now: {}",
            name.as_ref(),
            stats.wealth()
        );

        if stats.wealth() >= COMFORT_LEVEL {
            info!(
                "{}: WooHoo! Rich enough for now. Back home to mah li'lle lady",
                name.as_ref()
            );

            state_machine.change_state(
                entity,
                Self::GoHomeAndSleepTilRested,
                exit_events,
                enter_events,
            );
        } else {
            state_machine.change_state(
                entity,
                Self::EnterMineAndDigForNugget,
                exit_events,
                enter_events,
            );
        }
    }

    fn VisitBankAndDepositGold_exit(name: impl AsRef<str>) {
        info!("{}: Leavin' the bank", name.as_ref());
    }
}

impl MinerState {
    fn GoHomeAndSleepTilRested_enter(
        entity: Entity,
        name: impl AsRef<str>,
        miner: &mut Miner,
        wife: &MinerWife,
        message_dispatcher: &mut MessageDispatcher,
    ) {
        if miner.location != Location::Shack {
            info!("{}: Walkin' home", name.as_ref());

            miner.location = Location::Shack;

            message_dispatcher.dispatch_message(entity, wife.wife_id, Message::HiHoneyImHome);
        }
    }

    fn GoHomeAndSleepTilRested_execute(
        entity: Entity,
        name: impl AsRef<str>,
        stats: &mut Stats,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        if !stats.is_fatigued() {
            info!(
                "{}: What a God darn fantastic nap! Time to find more gold",
                name.as_ref()
            );

            state_machine.change_state(
                entity,
                Self::EnterMineAndDigForNugget,
                exit_events,
                enter_events,
            );
        } else {
            stats.rest();

            info!("{}: ZZZZ... ", name.as_ref());

            state_machine.change_state(
                entity,
                Self::GoHomeAndSleepTilRested,
                exit_events,
                enter_events,
            );
        }
    }

    fn GoHomeAndSleepTilRested_exit(name: impl AsRef<str>) {
        info!("{}: Leaving the house", name.as_ref());
    }

    fn GoHomeAndSleepTilRested_on_message(
        message: &Message,
        entity: Entity,
        name: impl AsRef<str>,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        match message {
            Message::StewIsReady => {
                let now = Utc::now();

                debug!("Message handled by {} at time: {}", name.as_ref(), now);
                info!("{}: Ok hun, ahm a-comin'!", name.as_ref());

                state_machine.change_state(entity, Self::EatStew, exit_events, enter_events);
            }
            _ => (),
        }
    }
}

impl MinerState {
    fn QuenchThirst_enter(name: impl AsRef<str>, miner: &mut Miner) {
        if miner.location != Location::Saloon {
            info!(
                "{}: Boy, ah sure is thusty! Walking to the saloon",
                name.as_ref()
            );

            miner.location = Location::Shack;
        }
    }

    fn QuenchThirst_execute(
        entity: Entity,
        name: impl AsRef<str>,
        stats: &mut Stats,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        if stats.is_thirsty() {
            stats.buy_and_drink_a_whiskey();

            info!("{}: That's mighty fine sippin liquer", name.as_ref());

            state_machine.change_state(
                entity,
                Self::EnterMineAndDigForNugget,
                exit_events,
                enter_events,
            );
        } else {
            unreachable!();
        }
    }

    fn QuenchThirst_exit(name: impl AsRef<str>) {
        info!("{}: Leaving the saloon, feelin' good", name.as_ref());
    }
}

impl MinerState {
    fn EatStew_enter(name: impl AsRef<str>) {
        info!("{}: Smells reaaal good Elsa!", name.as_ref());
    }

    fn EatStew_execute(
        entity: Entity,
        name: impl AsRef<str>,
        state_machine: &mut MinerStateMachine,
        exit_events: &mut EventWriter<MinerStateExitEvent>,
        enter_events: &mut EventWriter<MinerStateEnterEvent>,
    ) {
        info!("{}: Tastes reaaal good too!", name.as_ref());

        state_machine.revert_to_previous_state(entity, exit_events, enter_events);
    }

    fn EatStew_exit(name: impl AsRef<str>) {
        info!(
            "{}: Thankya li'lle lady. Ah better get back to whatever ah wuz doin'",
            name.as_ref()
        );
    }
}
