use bevy::prelude::*;

use crate::components::miner::*;
use crate::game::miner::*;

pub fn update(mut query: Query<&mut Stats>) {
    for mut stats in query.iter_mut() {
        stats.update();
    }
}

pub fn global_state_execute(mut query: Query<&Name, With<MinerStateMachine>>) {
    for name in query.iter_mut() {
        debug!("executing miner global state for {}", name.as_str());

        MinerState::execute_global();
    }
}

pub fn state_enter(
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

pub fn state_execute(
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

pub fn state_exit(
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
