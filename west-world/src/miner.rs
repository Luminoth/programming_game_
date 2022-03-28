#![allow(non_snake_case)]

use tracing::info;

use crate::entity::Entity;
use crate::location::Location;
use crate::state::{State, StateMachine};

const COMFORT_LEVEL: i64 = 5;
const MAX_NUGGETS: i64 = 3;
const THIRST_LEVEL: i64 = 5;
const TIREDNESS_THRESHOLD: i64 = 5;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MinerState {
    GlobalState,

    EnterMineAndDigForNugget,
    VisitBankAndDepositGold,
    GoHomeAndSleepTilRested,
    QuenchThirst,
}

impl State<MinerComponents> for MinerState {
    type StateMachine = MinerStateMachine;

    fn enter(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        miner: &mut MinerComponents,
    ) {
        match self {
            Self::GlobalState => Self::GlobalState_enter(entity, state_machine, miner),
            Self::EnterMineAndDigForNugget => {
                Self::EnterMineAndDigForNugget_enter(entity, state_machine, miner)
            }
            Self::VisitBankAndDepositGold => {
                Self::VisitBankAndDepositGold_enter(entity, state_machine, miner)
            }
            Self::GoHomeAndSleepTilRested => {
                Self::GoHomeAndSleepTilRested_enter(entity, state_machine, miner)
            }
            Self::QuenchThirst => Self::QuenchThirst_enter(entity, state_machine, miner),
        }
    }

    fn execute(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        miner: &mut MinerComponents,
    ) {
        match self {
            Self::GlobalState => Self::GlobalState_execute(entity, state_machine, miner),
            Self::EnterMineAndDigForNugget => {
                Self::EnterMineAndDigForNugget_execute(entity, state_machine, miner)
            }
            Self::VisitBankAndDepositGold => {
                Self::VisitBankAndDepositGold_execute(entity, state_machine, miner)
            }
            Self::GoHomeAndSleepTilRested => {
                Self::GoHomeAndSleepTilRested_execute(entity, state_machine, miner)
            }
            Self::QuenchThirst => Self::QuenchThirst_execute(entity, state_machine, miner),
        }
    }

    fn exit(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        miner: &mut MinerComponents,
    ) {
        match self {
            Self::GlobalState => Self::GlobalState_exit(entity, state_machine, miner),
            Self::EnterMineAndDigForNugget => {
                Self::EnterMineAndDigForNugget_exit(entity, state_machine, miner)
            }
            Self::VisitBankAndDepositGold => {
                Self::VisitBankAndDepositGold_exit(entity, state_machine, miner)
            }
            Self::GoHomeAndSleepTilRested => {
                Self::GoHomeAndSleepTilRested_exit(entity, state_machine, miner)
            }
            Self::QuenchThirst => Self::QuenchThirst_exit(entity, state_machine, miner),
        }
    }
}

impl MinerState {
    fn GlobalState_enter(
        _entity: &Entity,
        _: &mut MinerStateMachine,
        _miner: &mut MinerComponents,
    ) {
    }

    fn GlobalState_execute(
        _entity: &Entity,
        _state_machine: &mut MinerStateMachine,
        _miner: &mut MinerComponents,
    ) {
    }

    fn GlobalState_exit(_entity: &Entity, _: &mut MinerStateMachine, _miner: &mut MinerComponents) {
    }
}

impl MinerState {
    fn EnterMineAndDigForNugget_enter(
        entity: &Entity,
        _: &mut MinerStateMachine,
        miner: &mut MinerComponents,
    ) {
        if miner.location != Location::GoldMine {
            info!("{}: Walkin' to the gold mine", entity.name());

            miner.change_location(Location::GoldMine);
        }
    }

    fn EnterMineAndDigForNugget_execute(
        entity: &Entity,
        state_machine: &mut MinerStateMachine,
        miner: &mut MinerComponents,
    ) {
        miner.mine_gold();

        info!("{}: Pickin' up a nugget", entity.name());

        if miner.are_pockets_full() {
            state_machine.change_state(entity, Self::VisitBankAndDepositGold, miner);
        } else if miner.is_thirsty() {
            state_machine.change_state(entity, Self::QuenchThirst, miner);
        }
    }

    fn EnterMineAndDigForNugget_exit(
        entity: &Entity,
        _: &mut MinerStateMachine,
        _: &mut MinerComponents,
    ) {
        info!(
            "{}: Ah'm leavin' the gold mine with mah pockets full o' sweet gold",
            entity.name()
        )
    }
}

impl MinerState {
    fn VisitBankAndDepositGold_enter(
        entity: &Entity,
        _: &mut MinerStateMachine,
        miner: &mut MinerComponents,
    ) {
        if miner.location != Location::Bank {
            info!("{}: Goin' to the bank. Yes siree", entity.name());

            miner.change_location(Location::Bank)
        }
    }

    fn VisitBankAndDepositGold_execute(
        entity: &Entity,
        state_machine: &mut MinerStateMachine,
        miner: &mut MinerComponents,
    ) {
        miner.transfer_gold_to_wealth();

        info!(
            "{}: Depositing gold. Total savings now: {}",
            entity.name(),
            miner.wealth()
        );

        if miner.wealth() >= COMFORT_LEVEL {
            info!(
                "{}: WooHoo! Rich enough for now. Back home to mah li'lle lady",
                entity.name()
            );

            state_machine.change_state(entity, Self::GoHomeAndSleepTilRested, miner);
        } else {
            state_machine.change_state(entity, Self::EnterMineAndDigForNugget, miner);
        }
    }

    fn VisitBankAndDepositGold_exit(
        entity: &Entity,
        _: &mut MinerStateMachine,
        _: &mut MinerComponents,
    ) {
        info!("{}: Leavin' the bank", entity.name());
    }
}

impl MinerState {
    fn GoHomeAndSleepTilRested_enter(
        entity: &Entity,
        _: &mut MinerStateMachine,
        miner: &mut MinerComponents,
    ) {
        if miner.location != Location::Shack {
            info!("{}: Walkin' home", entity.name());

            miner.change_location(Location::Shack)
        }
    }

    fn GoHomeAndSleepTilRested_execute(
        entity: &Entity,
        state_machine: &mut MinerStateMachine,
        miner: &mut MinerComponents,
    ) {
        if !miner.is_fatigued() {
            info!(
                "{}: What a God darn fantastic nap! Time to find more gold",
                entity.name()
            );

            state_machine.change_state(entity, Self::EnterMineAndDigForNugget, miner);
        } else {
            miner.rest();

            info!("{}: ZZZZ... ", entity.name());

            state_machine.change_state(entity, Self::GoHomeAndSleepTilRested, miner);
        }
    }

    fn GoHomeAndSleepTilRested_exit(
        entity: &Entity,
        _: &mut MinerStateMachine,
        _: &mut MinerComponents,
    ) {
        info!("{}: Leaving the house", entity.name());
    }
}

impl MinerState {
    fn QuenchThirst_enter(entity: &Entity, _: &mut MinerStateMachine, miner: &mut MinerComponents) {
        if miner.location != Location::Saloon {
            info!(
                "{}: Boy, ah sure is thusty! Walking to the saloon",
                entity.name()
            );

            miner.change_location(Location::Shack)
        }
    }

    fn QuenchThirst_execute(
        entity: &Entity,
        state_machine: &mut MinerStateMachine,
        miner: &mut MinerComponents,
    ) {
        if miner.is_thirsty() {
            miner.buy_and_drink_a_whiskey();

            info!("{}: That's mighty fine sippin liquer", entity.name());

            state_machine.change_state(entity, Self::EnterMineAndDigForNugget, miner);
        } else {
            unreachable!();
        }
    }

    fn QuenchThirst_exit(entity: &Entity, _: &mut MinerStateMachine, _: &mut MinerComponents) {
        info!("{}: Leaving the saloon, feelin' good", entity.name());
    }
}

#[derive(Debug)]
struct MinerStateMachine {
    global_state: MinerState,

    current_state: MinerState,
    previous_state: Option<MinerState>,
}

impl Default for MinerStateMachine {
    fn default() -> Self {
        Self {
            global_state: MinerState::GlobalState,
            current_state: MinerState::GoHomeAndSleepTilRested,
            previous_state: None,
        }
    }
}

impl StateMachine<MinerComponents> for MinerStateMachine {
    type State = MinerState;

    fn update(&mut self, entity: &Entity, miner: &mut MinerComponents) {
        self.global_state.execute(entity, self, miner);

        self.current_state.execute(entity, self, miner);
    }

    fn change_state(
        &mut self,
        entity: &Entity,
        new_state: Self::State,
        miner: &mut MinerComponents,
    ) {
        self.previous_state = Some(self.current_state);

        self.current_state.exit(entity, self, miner);

        self.current_state = new_state;

        self.current_state.enter(entity, self, miner);
    }

    fn revert_to_previous_state(&mut self, entity: &Entity, miner: &mut MinerComponents) {
        self.change_state(entity, self.previous_state.unwrap(), miner);
    }
}

#[derive(Debug, Default)]
struct Stats {
    gold_carried: i64,
    money_in_bank: i64,
    thirst: i64,
    fatigue: i64,
}

#[derive(Debug)]
struct MinerComponents {
    location: Location,
    stats: Stats,
}

impl Default for MinerComponents {
    fn default() -> Self {
        Self {
            location: Location::Shack,
            stats: Stats::default(),
        }
    }
}

impl MinerComponents {
    fn update(&mut self) {
        self.stats.thirst += 1;
    }

    fn change_location(&mut self, location: Location) {
        self.location = location;
    }

    fn mine_gold(&mut self) {
        self.stats.gold_carried += 1;

        self.stats.fatigue += 1;
    }

    fn are_pockets_full(&self) -> bool {
        self.stats.gold_carried >= MAX_NUGGETS
    }

    fn transfer_gold_to_wealth(&mut self) {
        self.stats.money_in_bank += self.stats.gold_carried;

        self.stats.gold_carried = 0;
    }

    fn wealth(&self) -> i64 {
        self.stats.money_in_bank
    }

    fn rest(&mut self) {
        self.stats.fatigue -= 1;
    }

    fn is_fatigued(&self) -> bool {
        self.stats.fatigue > TIREDNESS_THRESHOLD
    }

    fn buy_and_drink_a_whiskey(&mut self) {
        self.stats.money_in_bank -= 2;

        self.stats.thirst = 0;
    }

    fn is_thirsty(&self) -> bool {
        self.stats.thirst >= THIRST_LEVEL
    }
}

#[derive(Debug)]
pub struct Miner {
    entity: Entity,
    state_machine: MinerStateMachine,
    components: MinerComponents,
}

impl Miner {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            entity: Entity::new(name),
            state_machine: MinerStateMachine::default(),
            components: MinerComponents::default(),
        }
    }

    pub fn update(&mut self) {
        self.components.update();

        self.state_machine
            .update(&self.entity, &mut self.components);
    }
}
