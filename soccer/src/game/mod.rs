use bevy::math::const_vec2;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

pub const BALL_RADIUS: f32 = 7.0;
pub const GOAL_BAR_WIDTH: f32 = 5.0;
pub const BORDER_WIDTH: f32 = 5.0;
pub const PLAYER_RADIUS: f32 = 15.0;
pub const PLAYER_SPREAD: Vec2 = const_vec2!([150.0, 100.0]);
pub const TEAM_SPREAD: f32 = 150.0;
pub const GOALIE_PAD: f32 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub enum Team {
    Red,
    Blue,
}

impl Default for Team {
    fn default() -> Self {
        Self::Red
    }
}

impl Team {
    pub fn color(&self) -> Color {
        match self {
            Self::Red => Color::RED,
            Self::Blue => Color::BLUE,
        }
    }
}
