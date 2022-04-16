pub mod debug;
pub mod messaging;

use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SimulationParams {
    pub pitch_extents: Vec2,
    pub goal_extents: Vec2,

    pub num_regions_horizontal: usize,
    pub num_regions_vertical: usize,

    // physics
    pub friction: f32,
}

#[derive(Debug, Default)]
pub struct GameState;
