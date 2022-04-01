use crate::components::state::StateMachine;

use super::state::State;

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

impl State for MinerState {
    fn execute_global(state_machine: &mut StateMachine<Self>) {}

    fn enter(self, state_machine: &mut StateMachine<Self>) {}

    fn execute(self, state_machine: &mut StateMachine<Self>) {}

    fn exit(self, state_machine: &mut StateMachine<Self>) {}
}
