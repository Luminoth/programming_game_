pub mod messaging;
pub mod state;
pub mod team;

use bevy::math::const_vec2;
use bevy::prelude::*;

pub const BALL_RADIUS: f32 = 7.0;
pub const GOAL_BAR_WIDTH: f32 = 5.0;
pub const BORDER_WIDTH: f32 = 5.0;
pub const PLAYER_RADIUS: f32 = 15.0;
pub const PLAYER_SPREAD: Vec2 = const_vec2!([150.0, 100.0]);
pub const TEAM_SPREAD: f32 = 150.0;
pub const GOALIE_PAD: f32 = 10.0;
