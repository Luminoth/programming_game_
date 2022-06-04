use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Weapon;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Blaster;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Shotgun;

#[derive(Debug, Default, Component, Inspectable)]
pub struct RocketLauncher;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Railgun;
