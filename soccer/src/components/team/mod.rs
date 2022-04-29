mod field_player;
mod goal_keeper;

pub use field_player::*;
pub use goal_keeper::*;

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
        players: &Query<(Entity, FieldPlayerQuery<T>, PhysicalQuery), Without<GoalKeeper>>,
        _goal_keeper: &Query<(Entity, GoalKeeperQuery<T>, PhysicalQuery), Without<FieldPlayer>>,
        closest: &Query<Entity, (With<T>, With<ClosestPlayer>)>,
    ) where
        T: TeamColorMarker,
    {
        if let Ok(closest) = closest.get_single() {
            commands.entity(closest).remove::<ClosestPlayer>();
        }

        let ball_position = ball_transform.translation.truncate();

        let mut closest_dist = f32::MAX;
        let mut closest_player = None;
        for (entity, player, physical) in players.iter() {
            let position = physical.transform.translation.truncate();
            let dist = position.distance_squared(ball_position);
            if dist < closest_dist {
                closest_dist = dist;
                closest_player = Some(entity);
            }
        }

        // TODO: goal keepers being closest is completely unaccounted for
        //let goal_keeper = goal_keeper.single();

        if let Some(closest_player) = closest_player {
            commands.entity(closest_player).insert(ClosestPlayer);
        }
    }

    pub fn reset_player_home_regions<T>(
        &self,
        players: &mut Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
        goal_keeper: &mut Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
        home_regions: [usize; TEAM_SIZE],
    ) where
        T: TeamColorMarker,
    {
        let mut goal_keeper = goal_keeper.single_mut();
        goal_keeper.goal_keeper.home_region = home_regions[0];

        let mut idx = 1;
        for mut player in players.iter_mut() {
            player.player.home_region = home_regions[idx];

            idx += 1;
        }
    }

    pub fn update_targets_of_waiting_players<T>(
        &self,
        pitch: &Pitch,
        players: &mut Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
        goal_keeper: &mut Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
    ) where
        T: TeamColorMarker,
    {
        for mut player in players.iter_mut() {
            if player.state_machine.is_in_state(FieldPlayerState::Wait)
                || player
                    .state_machine
                    .is_in_state(FieldPlayerState::ReturnToHomeRegion)
            {
                let target = pitch
                    .regions
                    .get(player.player.home_region)
                    .unwrap()
                    .position;
                player.steering.target = target;
            }
        }

        let mut goal_keeper = goal_keeper.single_mut();
        if goal_keeper
            .state_machine
            .is_in_state(GoalKeeperState::ReturnHome)
        {
            let target = pitch
                .regions
                .get(goal_keeper.goal_keeper.home_region)
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
        players: &Query<(AnyTeamFieldPlayerQuery, PhysicalQuery)>,
        controller: (FieldPlayerQueryItem<T>, &Transform),
        support: Option<(FieldPlayerQueryItem<T>, &Transform)>,
        ball_physical: &Physical,
        goal: GoalQueryItem<T>,
    ) where
        T: TeamColorMarker,
    {
        info!(
            "updating support spot for controlling team {:?}",
            team.team_color()
        );

        self.best_support_spot = None;

        let controller_position = controller.1.translation.truncate();

        let mut best_score = 0.0;
        let mut best_support_spot = None;
        for spot in &mut support_calculator.spots {
            spot.score = 1.0;

            // is it safe to pass to this spot?
            if self.is_pass_safe_from_all_opponents(
                params,
                team,
                controller_position,
                spot.position,
                None,
                players,
                ball_physical,
                params.max_passing_force,
            ) {
                spot.score += params.pass_safe_score;
            }

            // can we score a goal from this spot?
            if self
                .can_shoot(
                    params,
                    team,
                    spot.position,
                    &goal,
                    ball_physical,
                    players,
                    params.max_passing_force,
                )
                .is_some()
            {
                spot.score += params.can_score_score;
            }

            // how far away is our supporting player?
            if support.is_some() {
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
        team: &T,
        from: Vec2,
        target: Vec2,
        receiver: Option<&Transform>,
        players: &Query<(AnyTeamFieldPlayerQuery, PhysicalQuery)>,
        ball_physical: &Physical,
        passing_force: f32,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        for player in players.iter() {
            if !self.is_pass_safe_from_opponent(
                params,
                team,
                from,
                target,
                receiver,
                player,
                ball_physical,
                passing_force,
            ) {
                return false;
            }
        }

        true
    }

    fn is_pass_safe_from_opponent<T>(
        &self,
        params: &SimulationParams,
        team: &T,
        from: Vec2,
        target: Vec2,
        receiver: Option<&Transform>,
        opponent: (AnyTeamFieldPlayerQueryItem, PhysicalQueryItem),
        ball_physical: &Physical,
        passing_force: f32,
    ) -> bool
    where
        T: TeamColorMarker,
    {
        // ignore teammates
        if (opponent.0.red_team.is_some() && team.team_color() == TeamColor::Red)
            || (opponent.0.blue_team.is_some() && team.team_color() == TeamColor::Blue)
        {
            return true;
        }

        let opponent_position = opponent.1.transform.translation.truncate();

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
        let reach = opponent.1.physical.max_speed * time_for_ball + BALL_RADIUS + PLAYER_RADIUS;
        local_pos_opp.y.abs() >= reach
    }

    fn can_shoot<T>(
        &self,
        params: &SimulationParams,
        team: &T,
        from: Vec2,
        goal: &GoalQueryItem<T>,
        ball_physical: &Physical,
        players: &Query<(AnyTeamFieldPlayerQuery, PhysicalQuery)>,
        power: f32,
    ) -> Option<Vec2>
    where
        T: TeamColorMarker,
    {
        let mut rng = rand::thread_rng();

        let goal_position = goal.transform.translation.truncate();

        let mut num_attempts = params.num_attempts_to_find_valid_strike;
        while num_attempts > 0 {
            let mut target = goal_position + goal.goal.score_center;

            let min_y = goal_position.y + goal.goal.top.y + BALL_RADIUS;
            let max_y = goal_position.y + goal.goal.bottom.y - BALL_RADIUS;

            target.y = rng.gen_range(min_y..=max_y);

            let time = ball_physical.time_to_cover_distance(params, from, target, power);
            if time >= 0.0
                && self.is_pass_safe_from_all_opponents(
                    params,
                    team,
                    from,
                    target,
                    None,
                    players,
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
        team: &T,
        controller: Entity,
        controller_transform: &Transform,
        receiver: Entity,
        receiver_transform: &Transform,
        players: &Query<(AnyTeamFieldPlayerQuery, PhysicalQuery)>,
        ball_physical: &Physical,
        player_message_dispatcher: &mut FieldPlayerMessageDispatcher,
    ) where
        T: TeamColorMarker,
    {
        let controller_position = controller_transform.translation.truncate();
        let receiver_position = receiver_transform.translation.truncate();

        if self.is_pass_safe_from_all_opponents(
            params,
            team,
            controller_position,
            receiver_position,
            Some(receiver_transform),
            players,
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
