use bevy::prelude::*;

use crate::components::state_test::*;

pub fn state_test_idle_enter(mut query: Query<&TestStateIdleEnter>) {
    for _ in query.iter_mut() {
        info!("just gonna take a quick rest ...");
    }
}

pub fn state_test_idle_execute(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TestStateMachine), With<TestStateIdleExecute>>,
) {
    for (entity, mut machine) in query.iter_mut() {
        info!("standing around ...");
        machine.change_state(&mut commands, entity, TestState::Walk);
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
        machine.change_state(&mut commands, entity, TestState::Run);
    }
}

pub fn state_test_run_execute(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TestStateMachine), With<TestStateRunExecute>>,
) {
    for (entity, mut machine) in query.iter_mut() {
        info!("moving fast now!");
        machine.change_state(&mut commands, entity, TestState::Idle);
    }
}

pub fn state_test_run_exit(mut query: Query<&TestStateRunExit>) {
    for _ in query.iter_mut() {
        info!("phew, I'm tired!");
    }
}
