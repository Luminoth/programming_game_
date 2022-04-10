use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::Team;

#[derive(Debug, Default, Component, Inspectable)]
pub struct FieldPlayer {
    pub team: Team,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goalie {
    pub team: Team,
}
