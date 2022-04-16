use bevy::prelude::*;

use crate::components::wife::*;
use crate::events::messaging::MessageEvent;
use crate::game::wife::*;
use crate::resources::messaging::MessageDispatcher;

pub fn global_state_execute(
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut exit_events: EventWriter<WifeStateExitEvent>,
    mut query: Query<(Entity, WifeQuery)>,
) {
    for (entity, wife) in query.iter_mut() {
        debug!("executing wife global state for {}", wife.name.as_str());

        WifeState::execute_global(entity, wife, &mut exit_events, &mut enter_events);
    }
}

pub fn global_state_on_message(
    mut message_events: EventReader<(Entity, MessageEvent)>,
    mut exit_events: EventWriter<WifeStateExitEvent>,
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut query: Query<(Entity, WifeQuery)>,
) {
    for (receiver, event) in message_events.iter() {
        if let Ok((entity, wife)) = query.get_mut(*receiver) {
            debug!("global message for wife {}", wife.name.as_str());

            WifeState::on_message_global(event, entity, wife, &mut exit_events, &mut enter_events);
        }
    }
}

pub fn state_enter(
    mut events: EventReader<WifeStateEnterEvent>,
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<(Entity, WifeQuery)>,
) {
    for event in events.iter() {
        if let Ok((entity, wife)) = query.get_mut(event.entity()) {
            debug!(
                "entering wife state {:?} for {}",
                event.state(),
                wife.name.as_str()
            );

            event.state().enter(entity, wife, &mut message_dispatcher);
        }
    }
}

pub fn state_execute(
    mut exit_events: EventWriter<WifeStateExitEvent>,
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut query: Query<(Entity, WifeQuery)>,
) {
    for (entity, wife) in query.iter_mut() {
        debug!("executing wife state for {}", wife.name.as_str());

        wife.state_machine.current_state().execute(
            entity,
            wife,
            &mut exit_events,
            &mut enter_events,
        );
    }
}

pub fn state_exit(mut events: EventReader<WifeStateExitEvent>, mut query: Query<WifeQuery>) {
    for event in events.iter() {
        if let Ok(wife) = query.get_mut(event.entity()) {
            debug!("exiting wife state for {}", wife.name.as_str());

            event.state().exit(wife);
        }
    }
}

pub fn state_on_message(
    mut message_events: EventReader<(Entity, MessageEvent)>,
    mut exit_events: EventWriter<WifeStateExitEvent>,
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<(Entity, WifeQuery, &WifeMiner)>,
) {
    for (receiver, event) in message_events.iter() {
        if let Ok((entity, wife, miner)) = query.get_mut(*receiver) {
            debug!("message for wife {}", wife.name.as_str());

            wife.state_machine.current_state().on_message(
                event,
                entity,
                wife,
                miner,
                &mut exit_events,
                &mut enter_events,
                &mut message_dispatcher,
            );
        }
    }
}
