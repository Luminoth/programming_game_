use bevy::prelude::*;

use crate::game::state::State;

#[derive(Debug, Default, Component)]
pub struct StateMachine<T>
where
    T: State + Copy,
{
    current_state: T,
    previous_state: Option<T>,
}

impl<T> StateMachine<T>
where
    T: State + Copy,
{
    pub fn update(&mut self) {
        T::execute_global(self);

        self.current_state.execute(self);
    }

    pub fn change_state(&mut self, new_state: T) {
        self.current_state.exit(self);

        self.previous_state = Some(self.current_state);

        self.current_state = new_state;

        self.current_state.enter(self);
    }
}
