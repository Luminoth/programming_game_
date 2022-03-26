#![allow(non_snake_case)]

use tracing::{error, info};

use crate::entity::Entity;
use crate::location::Location;

const COMFORT_LEVEL: i64 = 5;
const MAX_NUGGETS: i64 = 3;
const THIRST_LEVEL: i64 = 5;
const TIREDNESS_THRESHOLD: i64 = 5;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    EnterMineAndDigForNugget,
    VisitBankAndDepositGold,
    GoHomeAndSleepTilRested,
    QuenchThirst,
}

impl State {
    fn enter(state: State, miner: &mut Miner) {
        match state {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_enter(miner),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_enter(miner),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_enter(miner),
            Self::QuenchThirst => Self::QuenchThirst_enter(miner),
        }
    }

    fn execute(state: State, miner: &mut Miner) {
        match state {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_execute(miner),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_execute(miner),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_execute(miner),
            Self::QuenchThirst => Self::QuenchThirst_execute(miner),
        }
    }

    fn exit(state: State, miner: &mut Miner) {
        match state {
            Self::EnterMineAndDigForNugget => Self::EnterMineAndDigForNugget_exit(miner),
            Self::VisitBankAndDepositGold => Self::VisitBankAndDepositGold_exit(miner),
            Self::GoHomeAndSleepTilRested => Self::GoHomeAndSleepTilRested_exit(miner),
            Self::QuenchThirst => Self::QuenchThirst_exit(miner),
        }
    }
}

impl State {
    fn EnterMineAndDigForNugget_enter(miner: &mut Miner) {
        if miner.location != Location::GoldMine {
            info!("{}: Walkin' to the gold mine", miner.entity.name());

            miner.change_location(Location::GoldMine);
        }
    }

    fn EnterMineAndDigForNugget_execute(miner: &mut Miner) {
        miner.mine_gold();

        info!("{}: Pickin' up a nugget", miner.entity.name());

        if miner.are_pockets_full() {
            miner.change_state(Self::VisitBankAndDepositGold);
        } else if miner.is_thirsty() {
            miner.change_state(Self::QuenchThirst);
        }
    }

    fn EnterMineAndDigForNugget_exit(miner: &mut Miner) {
        info!(
            "{}: Ah'm leavin' the gold mine with mah pockets full o' sweet gold",
            miner.entity.name()
        )
    }
}

impl State {
    fn VisitBankAndDepositGold_enter(miner: &mut Miner) {
        if miner.location != Location::Bank {
            info!("{}: Goin' to the bank. Yes siree", miner.entity.name());

            miner.change_location(Location::Bank)
        }
    }

    fn VisitBankAndDepositGold_execute(miner: &mut Miner) {
        miner.transfer_gold_to_wealth();

        info!(
            "{}: Depositing gold. Total savings now: {}",
            miner.entity.name(),
            miner.wealth()
        );

        if miner.wealth() >= COMFORT_LEVEL {
            info!(
                "{}: WooHoo! Rich enough for now. Back home to mah li'lle lady",
                miner.entity.name()
            );

            miner.change_state(Self::GoHomeAndSleepTilRested);
        } else {
            miner.change_state(Self::EnterMineAndDigForNugget);
        }
    }

    fn VisitBankAndDepositGold_exit(miner: &mut Miner) {
        info!("{}: Leavin' the bank", miner.entity.name());
    }
}

impl State {
    fn GoHomeAndSleepTilRested_enter(miner: &mut Miner) {
        if miner.location != Location::Shack {
            info!("{}: Walkin' home", miner.entity.name());

            miner.change_location(Location::Shack)
        }
    }

    fn GoHomeAndSleepTilRested_execute(miner: &mut Miner) {
        if !miner.is_fatigued() {
            info!(
                "{}: What a God darn fantastic nap! Time to find more gold",
                miner.entity.name()
            );

            miner.change_state(Self::EnterMineAndDigForNugget);
        } else {
            miner.rest();

            info!("{}: ZZZZ... ", miner.entity.name());

            miner.change_state(Self::GoHomeAndSleepTilRested);
        }
    }

    fn GoHomeAndSleepTilRested_exit(miner: &mut Miner) {
        info!("{}: Leaving the house", miner.entity.name());
    }
}

impl State {
    fn QuenchThirst_enter(miner: &mut Miner) {
        if miner.location != Location::Saloon {
            info!(
                "{}: Boy, ah sure is thusty! Walking to the saloon",
                miner.entity.name()
            );

            miner.change_location(Location::Shack)
        }
    }

    fn QuenchThirst_execute(miner: &mut Miner) {
        if miner.is_thirsty() {
            miner.buy_and_drink_a_whiskey();

            info!("{}: That's mighty fine sippin liquer", miner.entity.name());

            miner.change_state(Self::EnterMineAndDigForNugget);
        } else {
            error!("ERROR!");
        }
    }

    fn QuenchThirst_exit(miner: &mut Miner) {
        info!("{}: Leaving the saloon, feelin' good", miner.entity.name());
    }
}

#[derive(Debug)]
struct Stats {
    gold_carried: i64,
    money_in_bank: i64,
    thirst: i64,
    fatigue: i64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            gold_carried: 0,
            money_in_bank: 0,
            thirst: 0,
            fatigue: 0,
        }
    }
}

impl Stats {}

#[derive(Debug)]
pub struct Miner {
    entity: Entity,
    state: State,
    location: Location,
    stats: Stats,
}

impl Miner {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            entity: Entity::new(name),
            state: State::GoHomeAndSleepTilRested,
            location: Location::Shack,
            stats: Stats::default(),
        }
    }

    pub fn update(&mut self) {
        self.stats.thirst += 1;

        State::execute(self.state, self);
    }

    fn change_state(&mut self, new_state: State) {
        State::exit(self.state, self);

        self.state = new_state;

        State::enter(self.state, self);
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
        self.stats.fatigue >= TIREDNESS_THRESHOLD
    }

    fn buy_and_drink_a_whiskey(&mut self) {
        self.stats.money_in_bank -= 2;

        self.stats.thirst = 0;
    }

    fn is_thirsty(&self) -> bool {
        self.stats.thirst >= THIRST_LEVEL
    }
}
