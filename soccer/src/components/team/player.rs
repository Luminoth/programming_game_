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

    pub fn is_in_hot_region(
        &self,
        transform: &Transform,
        opponent_goal_transform: &Transform,
        pitch: &Pitch,
    ) -> bool {
        let position = transform.translation.truncate();
        let opponent_goal_position = opponent_goal_transform.translation.truncate();

        (position.y - opponent_goal_position.y).abs() < pitch.length() / 3.0
    }

    pub fn is_opponent_within_radius<T>(
        &self,
        transform: &Transform,
        opponents: &Query<&Transform, (With<SoccerPlayer>, Without<T>)>,
        radius: f32,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        let radius_squared = radius * radius;
        let position = transform.translation.truncate();
        for opponent_transform in opponents.iter() {
            let opponent_position = opponent_transform.translation.truncate();
            if position.distance_squared(opponent_position) < radius_squared {
                return true;
            }
        }

        false
    }

    pub fn find_support<T>(
        &self,
        commands: &mut Commands,
        message_dispatcher: &mut FieldPlayerMessageDispatcher,
        team: &SoccerTeam,
        field_players: &Query<(Entity, FieldPlayerQuery<T>, PhysicalQuery)>,
        supporting: Option<Entity>,
        controlling: Entity,
    ) where
        T: TeamColorMarker,
    {
        let best_supporting = team
            .determine_best_supporting_attacker(team, field_players, controlling)
            .unwrap();
        if let Some(supporting) = supporting {
            if best_supporting != supporting {
                commands.entity(supporting).remove::<SupportingPlayer>();
                message_dispatcher.dispatch_message(Some(supporting), FieldPlayerMessage::GoHome);

                commands.entity(best_supporting).insert(SupportingPlayer);
            }

            message_dispatcher
                .dispatch_message(Some(best_supporting), FieldPlayerMessage::SupportAttacker);
        } else {
            commands.entity(best_supporting).insert(SupportingPlayer);

            message_dispatcher
                .dispatch_message(Some(best_supporting), FieldPlayerMessage::SupportAttacker);
        }
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

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ReceivingPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct ReceivingPlayerQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub receiving: &'w ReceivingPlayer,
    pub team: &'w T,
    pub name: &'w Name,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ClosestPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct ClosestPlayerQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub closest: &'w ClosestPlayer,
    pub team: &'w T,
    pub name: &'w Name,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ControllingPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct ControllingPlayerQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub controlling: &'w ControllingPlayer,
    pub team: &'w T,
    pub name: &'w Name,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct SupportingPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct SupportingPlayerQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub supporting: &'w SupportingPlayer,
    pub team: &'w T,
    pub name: &'w Name,
}
