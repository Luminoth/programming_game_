use bevy::prelude::*;

use crate::events::state::{StateEnterEvent, StateExitEvent};
use crate::game::state::State;

#[derive(Debug, Default, Component)]
pub struct StateMachine<T>
where
    T: State,
{
    current_state: T,
    previous_state: Option<T>,
}

impl<T> StateMachine<T>
where
    T: State + Send + Sync + 'static,
{
    pub fn current_state(&self) -> T {
        self.current_state
    }

    pub fn change_state(
        &mut self,
        entity: Entity,
        new_state: T,
        exit_events: &mut EventWriter<StateExitEvent<T>>,
        enter_events: &mut EventWriter<StateEnterEvent<T>>,
    ) {
        exit_events.send(StateExitEvent::new(entity, self.current_state));

        self.previous_state = Some(self.current_state);

        self.current_state = new_state;

        enter_events.send(StateEnterEvent::new(entity, self.current_state));
    }

    pub fn revert_to_previous_state(
        &mut self,
        entity: Entity,
        exit_events: &mut EventWriter<StateExitEvent<T>>,
        enter_events: &mut EventWriter<StateEnterEvent<T>>,
    ) {
        self.change_state(
            entity,
            self.previous_state.unwrap(),
            exit_events,
            enter_events,
        );
    }
}
