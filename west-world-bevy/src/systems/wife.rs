use bevy::prelude::*;

use crate::components::wife::*;
use crate::game::wife::*;

pub fn update(time: Res<Time>, mut query: Query<&mut Wife>) {
    for mut wife in query.iter_mut() {
        wife.update(time.delta());
    }
}

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

pub fn state_enter(
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
