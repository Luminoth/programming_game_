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
        opponent_goal: &GoalQueryItem,
        pitch: &Pitch,
    ) -> bool {
        let position = transform.translation.truncate();
        let opponent_goal_center = opponent_goal.goal.get_score_center(opponent_goal.transform);

        (position.y - opponent_goal_center.y).abs() < pitch.length() / 3.0
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

    fn is_position_on_front_of_player(
        &self,
        transform: &Transform,
        physical: &Physical,
        position: Vec2,
    ) -> bool {
        let player_position = transform.translation.truncate();
        let to_subject = position - player_position;
        to_subject.dot(physical.heading) > 0.0
    }

    pub fn is_threatened<'a, O>(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        physical: &Physical,
        opponents: O,
    ) -> bool
    where
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        let position = transform.translation.truncate();
        for opponent in opponents {
            let opponent_position = opponent.0.transform.translation.truncate();

            // if the distance to the opponent is less than
            // our comfort range and they're in front of us
            // then we're threatened
            if self.is_position_on_front_of_player(transform, physical, opponent_position)
                && position.distance_squared(opponent_position) < params.player_comfort_zone_squared
            {
                return true;
            }
        }

        false
    }

    pub fn find_support<'a, T, M, O, F>(
        &self,
        commands: &mut Commands,
        params: &SimulationParams,
        message_dispatcher: &mut FieldPlayerMessageDispatcher,
        team: &mut SoccerTeamQueryMutItem<T>,
        support_calculator: &mut SupportSpotCalculator,
        teammates: M,
        opponents: F,
        supporting: Option<Entity>,
        controller: (Entity, &Transform),
        ball: (&Physical, &BoundingCircle),
        opponent_goal: &GoalQueryItem,
    ) where
        T: TeamColorMarker,
        M: Iterator<Item = (Entity, FieldPlayerQueryItem<'a, T>, PhysicalQueryItem<'a>)>,
        F: Fn() -> O + Copy,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        info!("looking for support");

        let best_supporting = team
            .team
            .determine_best_supporting_attacker(
                params,
                team.color,
                support_calculator,
                teammates,
                opponents,
                controller,
                supporting.is_some(),
                ball,
                opponent_goal,
            )
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
pub struct SoccerPlayerQuery<T>
where
    T: TeamColorMarker,
{
    pub player: &'static SoccerPlayer,
    pub team: &'static T,
    pub name: &'static Name,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SoccerPlayerQueryMut<T>
where
    T: TeamColorMarker,
{
    pub player: &'static mut SoccerPlayer,
    pub team: &'static T,
    pub name: &'static Name,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ReceivingPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct ReceivingPlayerQuery<T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub receiving: &'static ReceivingPlayer,
    pub team: &'static T,
    pub name: &'static Name,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ClosestPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct ClosestPlayerQuery<T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub closest: &'static ClosestPlayer,
    pub team: &'static T,
    pub name: &'static Name,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ControllingPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct ControllingPlayerQuery<T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub controlling: &'static ControllingPlayer,
    pub team: &'static T,
    pub name: &'static Name,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct SupportingPlayer;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct SupportingPlayerQuery<T>
where
    T: TeamColorMarker,
{
    pub entity: Entity,
    pub supporting: &'static SupportingPlayer,
    pub team: &'static T,
    pub name: &'static Name,
}
