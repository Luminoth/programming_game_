use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Projectile;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Bolt;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Pellet;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Rocket;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Slug;
