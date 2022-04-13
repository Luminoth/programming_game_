pub mod debug;

use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SimulationParams {
    pub pitch_extents: Vec2,
    pub goal_extents: Vec2,

    // physics
    pub friction: f32,
}

#[derive(Debug, Default)]
pub struct GameState;
