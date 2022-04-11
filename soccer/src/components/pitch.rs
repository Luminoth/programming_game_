use bevy::prelude::*;
use bevy_inspector_egui::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Pitch;

#[derive(Debug, Default, Component, Inspectable)]
pub struct PitchBorder;
