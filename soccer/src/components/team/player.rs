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

    pub fn is_opponent_within_radius<'a, T, O>(
        &self,
        transform: &Transform,
        opponents: O,
        radius: f32,
    ) -> bool
    where
        T: TeamColorMarker,
        O: Iterator<Item = &'a Transform>,
    {
        let radius_squared = radius * radius;
        let position = transform.translation.truncate();
        for opponent_transform in opponents {
            let opponent_position = opponent_transform.translation.truncate();
            if position.distance_squared(opponent_position) < radius_squared {
                return true;
            }
        }

        false
    }

    fn is_position_on_front_of_player(&self, physical: &PhysicalQueryItem, position: Vec2) -> bool {
        let player_position = physical.transform.translation.truncate();
        let to_subject = position - player_position;
        to_subject.dot(physical.physical.heading) > 0.0
    }

    pub fn is_threatened<'a, O>(
        &self,
        params: &SimulationParams,
        physical: &PhysicalQueryItem,
        opponents: O,
    ) -> bool
    where
        O: Iterator<Item = (&'a Actor, PhysicalQueryItem<'a>)>,
    {
        let position = physical.transform.translation.truncate();
        for opponent in opponents {
            let opponent_position = opponent.1.transform.translation.truncate();

            // if the distance to the opponent is less than
            // our comfort range and they're in front of us
            // then we're threatened
            if self.is_position_on_front_of_player(physical, opponent_position)
                && position.distance_squared(opponent_position) < params.player_comfort_zone_squared
            {
                return true;
            }
        }

        false
    }

    pub fn find_support<'a, T, M>(
        &self,
        commands: &mut Commands,
        message_dispatcher: &mut FieldPlayerMessageDispatcher,
        team: &mut SoccerTeam,
        teammates: M,
        supporting: Option<Entity>,
        controlling: Entity,
    ) where
        T: TeamColorMarker,
        M: Iterator<Item = (Entity, FieldPlayerQueryItem<'a, T>, PhysicalQueryItem<'a>)>,
    {
        info!("looking for support");

        let best_supporting = team
            .determine_best_supporting_attacker(teammates, controlling)
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
