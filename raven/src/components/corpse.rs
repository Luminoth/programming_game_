use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// TODO: pull corpse parameters from a config

#[derive(Debug, Component, Inspectable)]
pub struct Corpse {
    #[inspectable(ignore)]
    pub timer: Timer,
}

impl Default for Corpse {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(10.0, false),
        }
    }
}
