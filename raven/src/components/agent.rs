use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Agent;

#[derive(Debug, Default, Component)]
pub struct SelectedAgent;
