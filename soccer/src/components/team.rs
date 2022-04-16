use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::team::*;
use crate::resources::SimulationParams;

use super::state::StateMachine;

#[derive(Debug, Default, Component, Inspectable)]
pub struct SoccerTeam {
    pub team: Team,
}

pub type SoccerTeamStateMachine = StateMachine<SoccerTeamState>;

#[derive(Debug, Default, Clone, Copy, Component, Inspectable)]
pub struct SupportSpot {
    pub position: Vec2,
    pub score: f32,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct SupportSpotDebug;

#[derive(Debug, Clone, Component, Inspectable)]
pub struct SupportSpotCalculator {
    pub spots: Vec<SupportSpot>,
}

impl SupportSpotCalculator {
    pub fn new(team: Team, params: &SimulationParams) -> Self {
        let hw = params.pitch_extents.x * 0.5;
        let hh = params.pitch_extents.y * 0.5;

        let half_spots_horizontal = params.num_support_spots_horizontal / 2;
        let spot_count = half_spots_horizontal * params.num_support_spots_vertical;
        let spot_size = Vec2::new(
            hw / half_spots_horizontal as f32,
            params.pitch_extents.y / params.num_support_spots_vertical as f32,
        );
        let half_spot_size = spot_size * 0.5;

        let mut spots = Vec::with_capacity(spot_count);
        for y in 0..params.num_support_spots_vertical {
            for x in 0..half_spots_horizontal {
                let position = Vec2::new(
                    team.sign() * (-hw + (x as f32 * spot_size.x) + half_spot_size.x),
                    -hh + (y as f32 * spot_size.y) + half_spot_size.y,
                );
                spots.push(SupportSpot {
                    position,
                    ..Default::default()
                });
            }
        }

        Self { spots }
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct FieldPlayer {
    pub team: Team,
}

pub type FieldPlayerStateMachine = StateMachine<FieldPlayerState>;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goalie {
    pub team: Team,
}

pub type GoalieStateMachine = StateMachine<GoalieState>;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ReceivingPlayer;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ClosestPlayer;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ControllingPlayer;

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct SupportingPlayer;
