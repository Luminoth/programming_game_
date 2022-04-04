use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Physical {
    pub velocity: Vec2,
    pub heading: Vec2,
    pub side: Vec2,

    pub mass: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub max_turn_rate: f32,
}
