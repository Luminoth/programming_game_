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
    pub fn are_same_team<T>(team: &T, player: &AnyTeamSoccerPlayerQueryItem) -> bool
    where
        T: TeamColorMarker,
    {
        (player.red_team.is_some() && team.team_color() == TeamColor::Red)
            || (player.blue_team.is_some() && team.team_color() == TeamColor::Blue)
    }

    pub fn get_home_region<'a>(&self, pitch: &'a Pitch) -> &'a PitchRegion {
        pitch.regions.get(self.home_region).unwrap()
    }

    pub fn is_in_home_region(&self, transform: &Transform, pitch: &Pitch) -> bool {
        self.get_home_region(pitch)
            .is_inside_half(transform.translation.truncate())
    }

    pub fn is_in_hot_region<T>(
        &self,
        team: &T,
        transform: &Transform,
        goals: &Query<AnyTeamGoalQuery>,
        pitch: &Pitch,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        let position = transform.translation.truncate();
        let opponent_goal_position = Goal::get_opponent_goal_position(team, goals).unwrap();

        (position.y - opponent_goal_position.y).abs() < pitch.length() / 3.0
    }

    pub fn is_opponent_within_radius<T>(
        &self,
        team: &T,
        transform: &Transform,
        players: &Query<(AnyTeamSoccerPlayerQuery, PhysicalQuery)>,
        radius: f32,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        let radius_squared = radius * radius;
        let position = transform.translation.truncate();
        for (player, physical) in players.iter() {
            // ignore teammates
            if SoccerPlayer::are_same_team(team, &player) {
                return true;
            }

            let opponent_position = physical.transform.translation.truncate();
            if position.distance_squared(opponent_position) < radius_squared {
                return true;
            }
        }

        false
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
