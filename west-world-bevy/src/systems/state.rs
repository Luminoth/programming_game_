use bevy::prelude::*;

use crate::components::miner::MinerStateMachine;
use crate::components::wife::WifeStateMachine;

pub fn update_miner_global_state(mut query: Query<(&mut MinerStateMachine, &Name)>) {
    for (mut state_machine, name) in query.iter_mut() {
        info!("updating miner global state for {}", name.as_str());
    }
}

pub fn update_miner_state(mut query: Query<(&mut MinerStateMachine, &Name)>) {
    for (mut state_machine, name) in query.iter_mut() {
        info!("updating miner state for {}", name.as_str());
    }
}

pub fn update_wife_global_state(mut query: Query<(&mut WifeStateMachine, &Name)>) {
    for (mut state_machine, name) in query.iter_mut() {
        info!("updating wife global state for {}", name.as_str());
    }
}

pub fn update_wife_state(mut query: Query<(&mut WifeStateMachine, &Name)>) {
    for (mut state_machine, name) in query.iter_mut() {
        info!("updating wife state for {}", name.as_str());
    }
}
