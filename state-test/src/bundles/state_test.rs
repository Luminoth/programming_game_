use bevy::prelude::*;

use crate::components::state_test::*;

#[derive(Debug, Bundle)]
pub struct StateTestBundle {
    owner: StateMachineOwner,
}

impl StateTestBundle {
    pub fn spawn(commands: &mut Commands) {
        let mut bundle = commands.spawn_bundle(StateTestBundle {
            owner: StateMachineOwner::default(),
        });

        TestStateMachine::insert(&mut bundle, TestStates::Idle);
    }
}
