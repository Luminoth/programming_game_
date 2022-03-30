use bevy::prelude::*;

use crate::game::state::State;

#[derive(Debug, Default, Component)]
pub struct StateMachine<T>
where
    T: State,
{
    // TODO: global state
    current_state: T,
    previous_state: Option<T>,
}

impl<T> StateMachine<T>
where
    T: State + Default,
{
    // TODO:
    //pub fn update_global(&mut self) {}

    pub fn update(&mut self) {}
}
