use bevy::prelude::*;

use crate::game::Location;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
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

#[derive(Debug, Component)]
pub struct Miner {
    location: Location,
}

impl Default for Miner {
    fn default() -> Self {
        Self {
            location: Location::Shack,
        }
    }
}

impl Miner {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) {
        let name = name.into();
        info!("spawning miner {}", name);

        commands
            .spawn()
            .insert(Miner::default())
            .insert(MinerState::default())
            .insert(Name::new(name));
    }
}
