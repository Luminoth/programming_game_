use std::time::Duration;

use bevy::prelude::*;

use crate::game::wife::*;

use super::state::StateMachine;

pub type WifeStateMachine = StateMachine<WifeState>;

#[derive(Debug, Default, Component)]
pub struct Wife {
    cook_timer: Timer,
}

impl Wife {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) {
        let name = name.into();
        info!("spawning wife {}", name);

        commands
            .spawn()
            .insert(Wife::default())
            .insert(WifeStateMachine::default())
            .insert(Name::new(name));
    }

    pub fn update(&mut self, dt: Duration) {
        self.cook_timer.tick(dt);
    }

    pub fn start_cooking(&mut self) {
        self.cook_timer = Timer::from_seconds(1.5, false);
    }

    pub fn is_cooking(&self) -> bool {
        !self.cook_timer.finished()
    }

    pub fn finished_cooking(&self) {
        self.cook_timer.just_finished();
    }
}
