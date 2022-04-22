use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use crate::game::miner::*;
use crate::game::Location;

use super::state::impl_state_machine;

impl_state_machine!(
    Miner,
    EnterMineAndDigForNugget,
    VisitBankAndDepositGold,
    GoHomeAndSleepTilRested,
    QuenchThirst,
    EatStew
);

#[derive(Debug, Default, Component)]
pub struct Stats {
    gold_carried: i64,
    money_in_bank: i64,
    thirst: i64,
    fatigue: i64,
}

impl Stats {
    pub fn update(&mut self) {
        self.thirst += 1;
    }

    pub fn mine_gold(&mut self) {
        self.gold_carried += 1;

        self.fatigue += 1;
    }

    pub fn are_pockets_full(&self) -> bool {
        self.gold_carried >= MAX_NUGGETS
    }

    pub fn transfer_gold_to_wealth(&mut self) {
        self.money_in_bank += self.gold_carried;

        self.gold_carried = 0;
    }

    pub fn wealth(&self) -> i64 {
        self.money_in_bank
    }

    pub fn rest(&mut self) {
        self.fatigue -= 1;
    }

    pub fn is_fatigued(&self) -> bool {
        self.fatigue > TIREDNESS_THRESHOLD
    }

    pub fn buy_and_drink_a_whiskey(&mut self) {
        self.money_in_bank -= 2;

        self.thirst = 0;
    }

    pub fn is_thirsty(&self) -> bool {
        self.thirst >= THIRST_LEVEL
    }
}

// TODO: should the location be its own separate component?
#[derive(Debug, Component)]
pub struct Miner {
    pub location: Location,
}

impl Default for Miner {
    fn default() -> Self {
        Self {
            location: Location::Shack,
        }
    }
}

impl Miner {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) -> Entity {
        let name = name.into();
        info!("spawning miner {}", name);

        let mut entity = commands.spawn();
        entity
            .insert(Miner::default())
            .insert(Stats::default())
            .insert(Name::new(name));

        MinerStateMachine::insert(&mut entity, MinerState::GoHomeAndSleepTilRested);

        entity.id()
    }
}

// this is a separate component because we have to add it after spawning the entities
#[derive(Debug, Component)]
pub struct MinerWife {
    pub wife_id: Entity,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct MinerQuery<'w> {
    pub miner: &'w mut Miner,
    pub stats: &'w mut Stats,
    pub state_machine: &'w mut MinerStateMachine,
    pub name: &'w Name,
}
