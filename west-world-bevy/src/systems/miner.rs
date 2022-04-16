use bevy::prelude::*;

use crate::components::miner::*;
use crate::events::messaging::MessageEvent;
use crate::game::miner::*;
use crate::resources::messaging::MessageDispatcher;

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
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<(Entity, MinerQuery, &MinerWife)>,
) {
    for event in events.iter() {
        if let Ok((entity, miner, wife)) = query.get_mut(event.entity()) {
            debug!(
                "entering miner state {:?} for {}",
                event.state(),
                miner.name.as_str()
            );

            event
                .state()
                .enter(entity, miner, wife, &mut message_dispatcher);
        }
    }
}

pub fn state_execute(
    mut exit_events: EventWriter<MinerStateExitEvent>,
    mut enter_events: EventWriter<MinerStateEnterEvent>,
    mut query: Query<(Entity, MinerQuery)>,
) {
    for (entity, miner) in query.iter_mut() {
        debug!("executing miner state for {}", miner.name.as_str());

        miner.state_machine.current_state().execute(
            entity,
            miner,
            &mut exit_events,
            &mut enter_events,
        );
    }
}

pub fn state_exit(mut events: EventReader<MinerStateExitEvent>, mut query: Query<MinerQuery>) {
    for event in events.iter() {
        if let Ok(miner) = query.get_mut(event.entity()) {
            debug!("exiting miner state for {}", miner.name.as_str());

            event.state().exit(miner);
        }
    }
}

pub fn state_on_message(
    mut message_events: EventReader<(Entity, MessageEvent)>,
    mut exit_events: EventWriter<MinerStateExitEvent>,
    mut enter_events: EventWriter<MinerStateEnterEvent>,
    mut query: Query<(Entity, MinerQuery)>,
) {
    for (receiver, event) in message_events.iter() {
        if let Ok((entity, miner)) = query.get_mut(*receiver) {
            debug!("message for miner {}", miner.name.as_str());

            miner.state_machine.current_state().on_message(
                event,
                entity,
                miner,
                &mut exit_events,
                &mut enter_events,
            );
        }
    }
}
