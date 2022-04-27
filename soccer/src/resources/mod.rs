pub mod debug;
pub mod messaging;
pub mod pitch;

use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SimulationParams {
    pub pitch_extents: Vec2,
    pub goal_extents: Vec2,

    // regions
    pub num_regions_horizontal: usize,
    pub num_regions_vertical: usize,

    // support spots
    pub num_support_spots_horizontal: usize,
    pub num_support_spots_vertical: usize,

    // scoring
    pub pass_safe_score: f32,
    pub can_score_score: f32,
    pub distance_from_controller_player_score: f32,

    // players
    pub player_max_speed_without_ball: f32,
    pub player_max_speed_with_ball: f32,

    pub max_passing_force: f32,

    pub num_attempts_to_find_valid_strike: usize,

    pub ball_within_receiving_range_squared: f32,
    pub player_kicking_distance_squared: f32,
    pub keeper_in_ball_range_squared: f32,

    // physics
    pub friction: f32,

    // debug
    pub debug_vis: bool,
}

#[derive(Debug, Default)]
pub struct GameState;
