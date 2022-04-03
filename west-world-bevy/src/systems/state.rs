use bevy::prelude::*;

use crate::components::miner::{Miner, MinerStateMachine, Stats};
use crate::components::wife::{Wife, WifeStateMachine};
use crate::game::miner::{MinerState, MinerStateEnterEvent, MinerStateExitEvent};
use crate::game::wife::{WifeState, WifeStateEnterEvent, WifeStateExitEvent};

pub fn miner_update(mut query: Query<&mut Stats>) {
    for mut stats in query.iter_mut() {
        stats.update();
    }
}

pub fn miner_global_state_execute(mut query: Query<&Name, With<MinerStateMachine>>) {
    for name in query.iter_mut() {
        debug!("executing miner global state for {}", name.as_str());

        MinerState::execute_global();
    }
}

pub fn miner_state_enter(
    mut events: EventReader<MinerStateEnterEvent>,
    mut query: Query<(&mut Miner, &Name)>,
) {
    for event in events.iter() {
        if let Ok((mut miner, name)) = query.get_mut(event.entity()) {
            debug!(
                "entering miner state {:?} for {}",
                event.state(),
                name.as_str()
            );

            event.state().enter(name.as_str(), &mut miner);
        }
    }
}

pub fn miner_state_execute(
    mut exit_events: EventWriter<MinerStateExitEvent>,
    mut enter_events: EventWriter<MinerStateEnterEvent>,
    mut query: Query<(Entity, &mut MinerStateMachine, &mut Stats, &Name)>,
) {
    for (entity, mut state_machine, mut stats, name) in query.iter_mut() {
        debug!("executing miner state for {}", name.as_str());

        state_machine.current_state().execute(
            entity,
            name.as_str(),
            &mut stats,
            &mut state_machine,
            &mut exit_events,
            &mut enter_events,
        );
    }
}

pub fn miner_state_exit(
    mut events: EventReader<MinerStateExitEvent>,
    query: Query<&Name, With<MinerStateMachine>>,
) {
    for event in events.iter() {
        if let Ok(name) = query.get(event.entity()) {
            debug!("exiting miner state for {}", name.as_str());

            event.state().exit(name.as_str());
        }
    }
}

pub fn wife_update(time: Res<Time>, mut query: Query<&mut Wife>) {
    for mut wife in query.iter_mut() {
        wife.update(time.delta());
    }
}

pub fn wife_global_state_execute(
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut exit_events: EventWriter<WifeStateExitEvent>,
    mut query: Query<(Entity, &mut WifeStateMachine, &Name)>,
) {
    for (entity, mut state_machine, name) in query.iter_mut() {
        debug!("executing wife global state for {}", name.as_str());

        WifeState::execute_global(
            entity,
            name.as_str(),
            &mut state_machine,
            &mut exit_events,
            &mut enter_events,
        );
    }
}

pub fn wife_state_enter(
    mut events: EventReader<WifeStateEnterEvent>,
    mut query: Query<(&mut Wife, &Name)>,
) {
    for event in events.iter() {
        if let Ok((mut wife, name)) = query.get_mut(event.entity()) {
            debug!(
                "entering wife state {:?} for {}",
                event.state(),
                name.as_str()
            );

            event.state().enter(name.as_str(), &mut wife);
        }
    }
}

pub fn wife_state_execute(
    mut exit_events: EventWriter<WifeStateExitEvent>,
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut query: Query<(Entity, &mut WifeStateMachine, &Name)>,
) {
    for (entity, mut state_machine, name) in query.iter_mut() {
        debug!("executing wife state for {}", name.as_str());

        state_machine.current_state().execute(
            entity,
            name.as_str(),
            &mut state_machine,
            &mut exit_events,
            &mut enter_events,
        );
    }
}

pub fn wife_state_exit(mut events: EventReader<WifeStateExitEvent>, query: Query<&Name>) {
    for event in events.iter() {
        if let Ok(name) = query.get(event.entity()) {
            debug!("exiting wife state for {}", name.as_str());

            event.state().exit(name.as_str());
        }
    }
}
