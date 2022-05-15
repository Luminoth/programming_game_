pub mod debug;
pub mod messaging;
pub mod pitch;
pub mod ui;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize, TypeUuid)]
#[uuid = "5f64bebb-c12f-4863-9282-e7cb6c70d88b"]
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
    pub player_kick_frequency: usize,
    pub player_kick_accuracy: f32,

    // ball
    pub ball_mass: f32,
    //pub ball_max_force: f32,
    //pub ball_max_speed: f32,

    // kicking
    pub max_passing_force: f32,
    pub min_pass_distance: f32,
    pub goal_keeper_min_pass_distance: f32,
    pub max_shooting_force: f32,
    pub max_dribble_force: f32,

    pub num_attempts_to_find_valid_strike: usize,

    pub chance_of_using_arrive_type_receive_behavior: f32,
    pub chance_player_attempts_pot_shot: f32,

    // range checking
    pub ball_within_receiving_range: f32,
    #[serde(skip)]
    pub ball_within_receiving_range_squared: f32,

    pub player_in_target_range: f32,
    #[serde(skip)]
    pub player_in_target_range_squared: f32,

    pub player_kicking_distance: f32,
    #[serde(skip)]
    pub player_kicking_distance_squared: f32,

    pub player_comfort_zone: f32,
    #[serde(skip)]
    pub player_comfort_zone_squared: f32,

    pub goal_keeper_tending_distance: f32,

    pub keeper_in_ball_range: f32,
    #[serde(skip)]
    pub keeper_in_ball_range_squared: f32,

    pub goal_keeper_intercept_range: f32,
    #[serde(skip)]
    pub goal_keeper_intercept_range_squared: f32,

    pub pass_threat_radius: f32,

    // steering
    pub seek_weight: f32,
    pub arrive_weight: f32,
    pub pursuit_weight: f32,
    pub interpose_weight: f32,
    pub separation_weight: f32,
    pub view_distance: f32,

    // physics
    pub friction: f32,

    // debug
    pub debug_vis: bool,
}

pub struct SimulationParamsAsset {
    pub handle: Handle<SimulationParams>,
}

#[derive(Debug, Default)]
pub struct GameState {
    pub red_team_ready: bool,
    pub blue_team_ready: bool,

    pub red_team_score: usize,
    pub blue_team_score: usize,
}

impl GameState {
    pub fn is_game_on(&self) -> bool {
        self.red_team_ready && self.blue_team_ready
    }
}
