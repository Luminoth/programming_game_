use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::Team;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goal {
    pub team: Team,
    pub facing: Vec2,
}
