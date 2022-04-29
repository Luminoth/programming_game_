use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::resources::pitch::*;

use super::*;

// rename of the book's PlayerBase
#[derive(Debug, Default, Component, Inspectable)]
pub struct SoccerPlayer {
    pub number: usize,

    pub home_region: usize,
    pub default_region: usize,
}

impl SoccerPlayer {
    pub fn get_home_region<'a>(&self, pitch: &'a Pitch) -> &'a PitchRegion {
        pitch.regions.get(self.home_region).unwrap()
    }

    pub fn is_in_home_region(&self, transform: &Transform, pitch: &Pitch) -> bool {
        self.get_home_region(pitch)
            .is_inside_half(transform.translation.truncate())
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct SoccerPlayerQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub player: &'w SoccerPlayer,
    pub team: &'w T,
    pub name: &'w Name,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SoccerPlayerQueryMut<'w, T>
where
    T: TeamColorMarker,
{
    pub player: &'w mut SoccerPlayer,
    pub team: &'w T,
    pub name: &'w Name,
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct AnyTeamSoccerPlayerQuery<'w> {
    pub player: &'w SoccerPlayer,
    pub blue_team: Option<&'w BlueTeam>,
    pub red_team: Option<&'w RedTeam>,
    pub name: &'w Name,
}

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
