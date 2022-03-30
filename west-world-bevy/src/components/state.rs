use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct StateMachine;

#[derive(Debug, Default, Component)]
pub struct State<T> {
    global_state: T,

    current_state: T,
    previous_state: Option<T>,
}
