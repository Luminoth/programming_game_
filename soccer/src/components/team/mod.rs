mod field_player;
mod goal_keeper;
mod player;

pub use field_player::*;
pub use goal_keeper::*;
pub use player::*;

use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;
use rand::Rng;

use crate::components::goal::*;
use crate::components::physics::*;
use crate::game::team::*;
use crate::game::{BALL_RADIUS, PLAYER_RADIUS};
use crate::resources::pitch::*;
use crate::resources::SimulationParams;
use crate::util::point_to_world_space;

use super::state::impl_state_machine;

impl_state_machine!(SoccerTeam, PrepareForKickOff, Defending, Attacking);

#[derive(Debug, Default, Component, Inspectable)]
pub struct SoccerTeam {
    best_support_spot: Option<Vec2>,
}

impl SoccerTeam {
    pub fn get_best_support_spot(&self) -> Vec2 {
        // TODO: if self.best_support_spot is None we should
        // calculate the best supporting spot
        // ... except we don't have the data available here
        self.best_support_spot.unwrap()
    }

    pub fn calculate_closest_player_to_ball<T>(
        &self,
        commands: &mut Commands,
        ball_transform: &Transform,
        players: &Query<(Entity, &Transform), (With<SoccerPlayer>, With<T>)>,
        closest: Option<Entity>,
    ) where
        T: TeamColorMarker,
    {
        if let Some(closest) = closest {
            commands.entity(closest).remove::<ClosestPlayer>();
        }

        let ball_position = ball_transform.translation.truncate();

        let mut closest_dist = f32::MAX;
        let mut closest_player = None;
        for (entity, transform) in players.iter() {
            let position = transform.translation.truncate();
            let dist = position.distance_squared(ball_position);
            if dist < closest_dist {
                closest_dist = dist;
                closest_player = Some(entity);
            }
        }

        if let Some(closest_player) = closest_player {
            commands.entity(closest_player).insert(ClosestPlayer);
        }
    }

    pub fn reset_player_home_regions<T>(
        &self,
        field_players: &mut Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
        goal_keeper: &mut GoalKeeperQueryMutItem<T>,
        home_regions: [usize; TEAM_SIZE],
    ) where
        T: TeamColorMarker,
    {
        goal_keeper.player.home_region = home_regions[0];

        let mut idx = 1;
        for mut field_player in field_players.iter_mut() {
            field_player.player.home_region = home_regions[idx];

            idx += 1;
        }
    }

    pub fn update_targets_of_waiting_players<T>(
        &self,
        pitch: &Pitch,
        field_players: &mut Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
        goal_keeper: &mut GoalKeeperQueryMutItem<T>,
    ) where
        T: TeamColorMarker,
    {
        for mut field_player in field_players.iter_mut() {
            if field_player
                .state_machine
                .is_in_state(FieldPlayerState::Wait)
                || field_player
                    .state_machine
                    .is_in_state(FieldPlayerState::ReturnToHomeRegion)
            {
                let target = pitch
                    .regions
                    .get(field_player.player.home_region)
                    .unwrap()
                    .position;
                field_player.steering.target = target;
            }
        }

        if goal_keeper
            .state_machine
            .is_in_state(GoalKeeperState::ReturnHome)
        {
            let target = pitch
                .regions
                .get(goal_keeper.player.home_region)
                .unwrap()
                .position;
            goal_keeper.steering.target = target;
        }
    }

    pub fn determine_best_supporting_position<T>(
        &mut self,
        params: &SimulationParams,
        team: &T,
        support_calculator: &mut SupportSpotCalculator,
        opponents: &Query<PhysicalQuery, (With<SoccerPlayer>, Without<T>)>,
        controller_transform: &Transform,
        have_support: bool,
        ball_physical: &Physical,
        opponent_goal: (&Goal, &Transform),
    ) where
        T: TeamColorMarker,
    {
        info!(
            "updating support spot for controlling team {:?}",
            team.team_color()
        );

        self.best_support_spot = None;

        let controller_position = controller_transform.translation.truncate();

        let mut best_score = 0.0;
        let mut best_support_spot = None;
        for spot in &mut support_calculator.spots {
            spot.score = 1.0;

            // is it safe to pass to this spot?
            if self.is_pass_safe_from_all_opponents(
                params,
                controller_position,
                spot.position,
                None,
                opponents,
                ball_physical,
                params.max_passing_force,
            ) {
                spot.score += params.pass_safe_score;
            }

            // can we score a goal from this spot?
            if self
                .can_shoot(
                    params,
                    spot.position,
                    opponent_goal,
                    ball_physical,
                    opponents,
                    params.max_passing_force,
                )
                .is_some()
            {
                spot.score += params.can_score_score;
            }

            // how far away is our supporting player?
            if have_support {
                let optimal_distance = 200.0;
                let dist = controller_position.distance(spot.position);
                let temp = (optimal_distance - dist).abs();
                if temp < optimal_distance {
                    spot.score += params.distance_from_controller_player_score
                        * (optimal_distance - temp)
                        / optimal_distance;
                }
            }

            // is this the best score?
            if spot.score > best_score {
                best_score = spot.score;
                best_support_spot = Some(spot.position);
            }
        }

        self.best_support_spot = best_support_spot;
    }

    fn is_pass_safe_from_all_opponents<T>(
        &self,
        params: &SimulationParams,
        from: Vec2,
        target: Vec2,
        receiver: Option<&Transform>,
        opponents: &Query<PhysicalQuery, (With<SoccerPlayer>, Without<T>)>,
        ball_physical: &Physical,
        passing_force: f32,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        for opponent in opponents.iter() {
            if !self.is_pass_safe_from_opponent(
                params,
                from,
                target,
                receiver,
                &opponent,
                ball_physical,
                passing_force,
            ) {
                return false;
            }
        }

        true
    }

    fn is_pass_safe_from_opponent(
        &self,
        params: &SimulationParams,
        from: Vec2,
        target: Vec2,
        receiver: Option<&Transform>,
        opponent: &PhysicalQueryItem,
        ball_physical: &Physical,
        passing_force: f32,
    ) -> bool {
        let opponent_position = opponent.transform.translation.truncate();

        let to_target = target - from;
        let to_target_norm = to_target.normalize_or_zero();

        let local_pos_opp = point_to_world_space(
            opponent_position,
            to_target_norm,
            to_target_norm.perp(),
            from,
        );

        // ignore opponents behind us
        if local_pos_opp.x < 0.0 {
            return true;
        }

        // is opponent closer to us than the target?
        // TODO: this logic is suspect ...
        if from.distance_squared(target) < opponent_position.distance_squared(from) {
            // can the receiver get there first?
            if let Some(receiver) = receiver {
                let receiver_position = receiver.translation.truncate();
                return target.distance_squared(opponent_position)
                    < target.distance_squared(receiver_position);
            } else {
                return true;
            }
        }

        let time_for_ball = ball_physical.time_to_cover_distance(
            params,
            Vec2::ZERO,
            Vec2::new(local_pos_opp.x, 0.0),
            passing_force,
        );

        // can the opponent intercept the ball in flight?
        let reach = opponent.physical.max_speed * time_for_ball + BALL_RADIUS + PLAYER_RADIUS;
        local_pos_opp.y.abs() >= reach
    }

    pub fn can_shoot<T>(
        &self,
        params: &SimulationParams,
        from: Vec2,
        opponent_goal: (&Goal, &Transform),
        ball_physical: &Physical,
        opponents: &Query<PhysicalQuery, (With<SoccerPlayer>, Without<T>)>,
        power: f32,
    ) -> Option<Vec2>
    where
        T: TeamColorMarker,
    {
        let mut rng = rand::thread_rng();

        let goal_position = opponent_goal.1.translation.truncate();

        let mut num_attempts = params.num_attempts_to_find_valid_strike;
        while num_attempts > 0 {
            let mut target = goal_position + opponent_goal.0.score_center;

            let min_y = goal_position.y + opponent_goal.0.top.y + BALL_RADIUS;
            let max_y = goal_position.y + opponent_goal.0.bottom.y - BALL_RADIUS;

            target.y = rng.gen_range(min_y..=max_y);

            let time = ball_physical.time_to_cover_distance(params, from, target, power);
            if time >= 0.0
                && self.is_pass_safe_from_all_opponents(
                    params,
                    from,
                    target,
                    None,
                    opponents,
                    ball_physical,
                    power,
                )
            {
                return Some(target);
            }

            num_attempts -= 1;
        }

        None
    }

    pub fn request_pass<T>(
        &self,
        params: &SimulationParams,
        controller: Entity,
        controller_transform: &Transform,
        receiver: Entity,
        receiver_transform: &Transform,
        opponents: &Query<PhysicalQuery, (With<SoccerPlayer>, Without<T>)>,
        ball_physical: &Physical,
        player_message_dispatcher: &mut FieldPlayerMessageDispatcher,
    ) where
        T: TeamColorMarker,
    {
        let controller_position = controller_transform.translation.truncate();
        let receiver_position = receiver_transform.translation.truncate();

        if self.is_pass_safe_from_all_opponents(
            params,
            controller_position,
            receiver_position,
            Some(receiver_transform),
            opponents,
            ball_physical,
            params.max_passing_force,
        ) {
            player_message_dispatcher.dispatch_message(
                Some(controller),
                FieldPlayerMessage::PassToMe(receiver, receiver_position),
            );
        }
    }
}

pub trait TeamColorMarker: std::fmt::Debug + Default + Component + Inspectable {
    fn team_color(&self) -> TeamColor;
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct RedTeam;

impl TeamColorMarker for RedTeam {
    fn team_color(&self) -> TeamColor {
        TeamColor::Red
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct BlueTeam;

impl TeamColorMarker for BlueTeam {
    fn team_color(&self) -> TeamColor {
        TeamColor::Blue
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct SoccerTeamQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub team: &'w SoccerTeam,
    pub color: &'w T,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SoccerTeamQueryMut<'w, T>
where
    T: TeamColorMarker,
{
    pub team: &'w mut SoccerTeam,
    pub color: &'w T,

    pub state_machine: &'w mut SoccerTeamStateMachine,
}

#[derive(Debug, Default, Clone, Copy, Component, Inspectable)]
pub struct SupportSpot {
    pub position: Vec2,
    pub score: f32,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct SupportSpotDebug;

#[derive(Debug, Component, Inspectable)]
pub struct SupportSpotCalculator {
    pub spots: Vec<SupportSpot>,
}

impl SupportSpotCalculator {
    pub fn new(team_color: TeamColor, params: &SimulationParams) -> Self {
        let goal_half_extents = params.goal_extents * 0.5;
        let hw = params.pitch_extents.x * 0.5 - goal_half_extents.x;
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
                    team_color.sign() * (-hw + (x as f32 * spot_size.x) + half_spot_size.x),
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
