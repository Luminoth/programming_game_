#![allow(non_snake_case)]

mod field_player;

pub use field_player::*;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

pub const TEAM_SIZE: usize = 5;

// first region is the goal keeper home
pub const BLUE_TEAM_DEFENDING_HOME_REGIONS: [usize; TEAM_SIZE] = [1, 6, 8, 3, 5];
pub const RED_TEAM_DEFENDING_HOME_REGIONS: [usize; TEAM_SIZE] = [16, 9, 11, 12, 14];
pub const BLUE_TEAM_ATTACKING_HOME_REGIONS: [usize; TEAM_SIZE] = [1, 4, 6, 12, 14];
pub const RED_TEAM_ATTACKING_HOME_REGIONS: [usize; TEAM_SIZE] = [16, 3, 5, 9, 13];

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

    #[allow(dead_code)]
    pub fn sign(&self) -> f32 {
        match self {
            Self::Red => -1.0,
            Self::Blue => 1.0,
        }
    }
}
