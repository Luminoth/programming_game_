use bevy::prelude::*;

use crate::components::wife::*;
use crate::events::messaging::MessageEvent;
use crate::game::wife::*;
use crate::resources::messaging::MessageDispatcher;

pub fn global_state_execute(
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

pub fn global_state_on_message(
    mut message_events: EventReader<MessageEvent>,
    mut exit_events: EventWriter<WifeStateExitEvent>,
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut query: Query<(Entity, &mut WifeStateMachine, &Name)>,
) {
    for event in message_events.iter() {
        if let Ok((entity, mut state_machine, name)) = query.get_mut(event.receiver) {
            debug!("global message for wife {}", name.as_str());

            WifeState::on_message_global(
                &event.message,
                entity,
                name.as_str(),
                &mut state_machine,
                &mut exit_events,
                &mut enter_events,
            );
        }
    }
}

pub fn state_enter(
    mut events: EventReader<WifeStateEnterEvent>,
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<(Entity, &mut Wife, &Name)>,
) {
    for event in events.iter() {
        if let Ok((entity, mut wife, name)) = query.get_mut(event.entity()) {
            debug!(
                "entering wife state {:?} for {}",
                event.state(),
                name.as_str()
            );

            event
                .state()
                .enter(entity, name.as_str(), &mut wife, &mut message_dispatcher);
        }
    }
}

pub fn state_execute(
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

pub fn state_exit(
    mut events: EventReader<WifeStateExitEvent>,
    query: Query<&Name, With<WifeStateMachine>>,
) {
    for event in events.iter() {
        if let Ok(name) = query.get(event.entity()) {
            debug!("exiting wife state for {}", name.as_str());

            event.state().exit(name.as_str());
        }
    }
}

pub fn state_on_message(
    mut message_events: EventReader<MessageEvent>,
    /*mut exit_events: EventWriter<WifeStateExitEvent>,
    mut enter_events: EventWriter<WifeStateEnterEvent>,
    mut message_events_writer: EventWriter<MessageEvent>,
    mut message_dispatcher: ResMut<MessageDispatcher>,*/
    mut query: Query<(Entity, &mut Wife, &mut WifeStateMachine, &Name)>,
) {
    for event in message_events.iter() {
        if let Ok((_entity, mut _wife, mut _state_machine, name)) = query.get_mut(event.receiver) {
            debug!("message for wife {}", name.as_str());

            /*state_machine.current_state().on_message(
                &event.message,
                entity,
                name.as_str(),
                &mut wife,
                &mut state_machine,
                &mut exit_events,
                &mut enter_events,
                &mut message_dispatcher,
                &mut message_events_writer,
            );*/
        }
    }
}
