use bevy::prelude::*;

use crate::components::state::StateMachine;
use crate::game::state::State;

pub fn update_state<T>(mut query: Query<(&mut StateMachine<T>, &Name)>)
where
    T: State + Send + Sync + 'static,
{
    for (mut state_machine, name) in query.iter_mut() {
        info!("updating state machine for {}", name.as_str());

        state_machine.update();
    }
}
