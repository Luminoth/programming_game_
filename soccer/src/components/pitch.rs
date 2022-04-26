use bevy::prelude::*;
use bevy_inspector_egui::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct PitchRegionDebug;

#[derive(Debug, Default, Component, Inspectable)]
pub struct PitchBorder;
