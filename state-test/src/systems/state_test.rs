use bevy::prelude::*;

use crate::components::state_test::*;

pub fn state_test_idle_execute(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TestStateMachine), With<TestStateIdleExecute>>,
) {
    for (entity, mut machine) in query.iter_mut() {
        info!("standing around ...");
        machine.change_state(&mut commands, entity, TestStates::Walk);
    }
}

pub fn state_test_walk_enter(mut query: Query<&TestStateWalkEnter>) {
    for _ in query.iter_mut() {
        info!("best be on my way!");
    }
}

pub fn state_test_walk_execute(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TestStateMachine), With<TestStateWalkExecute>>,
) {
    for (entity, mut machine) in query.iter_mut() {
        info!("what a nice day!");
        machine.change_state(&mut commands, entity, TestStates::Run);
    }
}

pub fn state_test_run_execute(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TestStateMachine), With<TestStateRunExecute>>,
) {
    for (entity, mut machine) in query.iter_mut() {
        info!("moving fast now!");
        machine.change_state(&mut commands, entity, TestStates::Idle);
    }
}

pub fn state_test_run_exit(mut query: Query<&TestStateWalkExit>) {
    for _ in query.iter_mut() {
        info!("phew, I'm tired!");
    }
}
