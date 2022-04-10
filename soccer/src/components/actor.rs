use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// rename of the book's BaseGameEntity
#[derive(Debug, Default, Component, Inspectable)]
pub struct Actor {
    pub bounding_radius: f32,
}
