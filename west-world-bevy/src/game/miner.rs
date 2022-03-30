use crate::game::state::State;

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
