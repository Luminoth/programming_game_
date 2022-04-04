use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Physical {
    pub velocity: Vec2,
    pub heading: Vec2,
    pub side: Vec2,
}
