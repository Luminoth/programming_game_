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
    pub player_mass: f32,
    pub player_max_force: f32,
    pub player_max_speed_without_ball: f32,
    pub player_max_speed_with_ball: f32,
    pub player_max_turn_rate: f32,

    // ball
    pub ball_mass: f32,
    pub ball_max_force: f32,
    pub ball_max_speed: f32,

    pub max_passing_force: f32,

    pub num_attempts_to_find_valid_strike: usize,

    pub chance_of_using_arrive_type_receive_behavior: f32,

    // range checking
    pub ball_within_receiving_range_squared: f32,
    pub player_in_target_range_squared: f32,
    pub player_kicking_distance_squared: f32,
    pub keeper_in_ball_range_squared: f32,
    pub pass_threat_radius: f32,

    // steering
    pub seek_weight: f32,
    pub arrive_weight: f32,
    pub pursuit_weight: f32,

    // physics
    pub friction: f32,

    // debug
    pub debug_vis: bool,
}

#[derive(Debug, Default)]
pub struct GameState {
    pub red_team_ready: bool,
    pub blue_team_ready: bool,
}

impl GameState {
    pub fn is_game_on(&self) -> bool {
        self.red_team_ready && self.blue_team_ready
    }
}
