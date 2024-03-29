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
use crate::resources::pitch::*;
use crate::resources::SimulationParams;
use crate::util::{get_tangent_points, point_to_world_space};

use super::state::impl_state_machine;

impl_state_machine!(SoccerTeam, PrepareForKickOff, Defending, Attacking);

#[derive(Debug, Default, Component, Inspectable)]
pub struct SoccerTeam {
    pub best_support_spot: Option<Vec2>,
}

impl SoccerTeam {
    pub fn send_all_field_players_home<F>(
        player_message_dispatcher: &mut FieldPlayerMessageDispatcher,
        field_players: F,
    ) where
        F: Iterator<Item = Entity>,
    {
        for field_player in field_players {
            player_message_dispatcher
                .dispatch_message(Some(field_player), FieldPlayerMessage::GoHome);
        }
    }

    pub fn calculate_closest_player_to_ball<'a, T, P>(
        &self,
        commands: &mut Commands,
        ball_position: Vec2,
        players: P,
        closest: Option<Entity>,
    ) where
        T: TeamColorMarker,
        P: Iterator<Item = (Entity, &'a Transform)>,
    {
        if let Some(closest) = closest {
            commands.entity(closest).remove::<ClosestPlayer>();
        }

        let mut closest_dist = f32::MAX;
        let mut closest_player = None;
        for (entity, transform) in players {
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
    }

    pub fn determine_best_supporting_position<'a, T, O, F>(
        &mut self,
        params: &SimulationParams,
        team: &T,
        support_calculator: &mut SupportSpotCalculator,
        opponents: F,
        controller_transform: &Transform,
        have_support: bool,
        ball: (&Physical, &BoundingCircle),
        opponent_goal: &GoalQueryItem,
    ) where
        T: TeamColorMarker,
        F: Fn() -> O + Copy,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        info!(
            "updating support spot for controlling team {:?}",
            team.team_color()
        );

        self.best_support_spot = None;

        let controller_position = controller_transform.translation.truncate();

        let mut best_score = 0.0;
        for spot in &mut support_calculator.spots {
            spot.score = 1.0;

            // is it safe to pass to this spot?
            if self.is_pass_safe_from_all_opponents::<T, O>(
                params,
                controller_position,
                spot.position,
                None,
                opponents(),
                ball,
                params.max_passing_force,
            ) {
                spot.score += params.pass_safe_score;
            }

            // can we score a goal from this spot?
            if self
                .can_shoot::<T, O, F>(
                    params,
                    spot.position,
                    opponent_goal,
                    ball,
                    opponents,
                    params.max_shooting_force,
                )
                .1
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
                self.best_support_spot = Some(spot.position);
            }
        }
    }

    fn is_pass_safe_from_all_opponents<'a, T, O>(
        &self,
        params: &SimulationParams,
        from: Vec2,
        target: Vec2,
        receiver: Option<&Transform>,
        opponents: O,
        ball: (&Physical, &BoundingCircle),
        passing_force: f32,
    ) -> bool
    where
        T: TeamColorMarker,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        for opponent in opponents {
            if !self.is_pass_safe_from_opponent(
                params,
                from,
                target,
                receiver,
                opponent,
                ball,
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
        opponent: (PhysicalQueryItem, &BoundingCircle),
        ball: (&Physical, &BoundingCircle),
        passing_force: f32,
    ) -> bool {
        let opponent_position = opponent.0.transform.translation.truncate();

        let to_target = target - from;
        let to_target_norm = to_target.normalize_or_zero();

        let local_pos_opp = point_to_world_space(
            opponent_position,
            to_target_norm,
            to_target_norm.perp(),
            from,
        );

        // ignore opponents behind us
        // TODO: this should be less than our negative bounding radius
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

        let time_for_ball = ball.0.time_to_cover_distance(
            params,
            Vec2::ZERO,
            Vec2::new(local_pos_opp.x, 0.0),
            passing_force,
        );

        // can the opponent intercept the ball in flight?
        let reach =
            opponent.0.physical.max_speed * time_for_ball + ball.1.radius + opponent.1.radius;
        local_pos_opp.y.abs() >= reach
    }

    pub fn determine_best_supporting_attacker<'a, T, M, O, F>(
        &mut self,
        params: &SimulationParams,
        team: &T,
        support_calculator: &mut SupportSpotCalculator,
        teammates: M,
        opponents: F,
        controller: (Entity, &Transform),
        have_support: bool,
        ball: (&Physical, &BoundingCircle),
        opponent_goal: &GoalQueryItem,
    ) -> Option<Entity>
    where
        T: TeamColorMarker,
        M: Iterator<Item = (Entity, FieldPlayerQueryItem<'a, T>, PhysicalQueryItem<'a>)>,
        F: Fn() -> O + Copy,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        info!("finding supporting attacker");

        let best_support_spot = if let Some(best_support_spot) = self.best_support_spot {
            best_support_spot
        } else {
            self.determine_best_supporting_position(
                params,
                team,
                support_calculator,
                opponents,
                controller.1,
                have_support,
                ball,
                opponent_goal,
            );
            self.best_support_spot.unwrap()
        };

        let mut closest = f32::MAX;
        let mut best_supporting = None;

        for (entity, field_player, physical) in teammates {
            // only attackers can support
            if field_player.field_player.role != FieldPlayerRole::Attacker || entity == controller.0
            {
                continue;
            }

            let position = physical.transform.translation.truncate();
            let dist = position.distance_squared(best_support_spot);
            if dist < closest {
                closest = dist;
                best_supporting = Some(entity);
            }
        }

        best_supporting
    }

    pub fn can_shoot<'a, T, O, F>(
        &self,
        params: &SimulationParams,
        from: Vec2,
        opponent_goal: &GoalQueryItem,
        ball: (&Physical, &BoundingCircle),
        opponents: F,
        power: f32,
    ) -> (Vec2, bool)
    where
        T: TeamColorMarker,
        F: Fn() -> O,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        let mut rng = rand::thread_rng();

        let top = opponent_goal.goal.get_top(opponent_goal.transform);
        let bottom = opponent_goal.goal.get_bottom(opponent_goal.transform);
        let center = opponent_goal.goal.get_score_center(opponent_goal.transform);

        let mut target = Vec2::ZERO;

        let mut num_attempts = params.num_attempts_to_find_valid_strike;
        while num_attempts > 0 {
            target = center;

            let min_y = bottom.y + ball.1.radius;
            let max_y = top.y - ball.1.radius;

            target.y = rng.gen_range(min_y..=max_y);

            let time = ball.0.time_to_cover_distance(params, from, target, power);
            if time >= 0.0
                && self.is_pass_safe_from_all_opponents::<T, O>(
                    params,
                    from,
                    target,
                    None,
                    opponents(),
                    ball,
                    power,
                )
            {
                return (target, true);
            }

            num_attempts -= 1;
        }

        (target, false)
    }

    pub fn find_pass<'a, T, M, O, F>(
        &self,
        params: &SimulationParams,
        passer: (Entity, &Transform),
        teammates: M,
        opponents: F,
        opponent_goal: &GoalQueryItem,
        ball: (&Physical, &Transform, &BoundingCircle),
        power: f32,
        min_passing_distance: f32,
    ) -> (Option<Entity>, Vec2)
    where
        T: TeamColorMarker,
        M: Iterator<Item = (Entity, FieldPlayerQueryItem<'a, T>, PhysicalQueryItem<'a>)>,
        F: Fn() -> O + Copy,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        let passer_position = passer.1.translation.truncate();
        let opponent_goal_center = opponent_goal.goal.get_score_center(opponent_goal.transform);
        let min_passing_distance_squared = min_passing_distance * min_passing_distance;

        let mut closest_goal = f32::MAX;
        let mut pass_target = Vec2::ZERO;
        let mut receiver = None;

        for (entity, _, physical) in teammates {
            // make sure the receiver is not the passer and
            // is further than the min passing distance
            let position = physical.transform.translation.truncate();
            if passer.0 == entity
                || passer_position.distance_squared(position) <= min_passing_distance_squared
            {
                continue;
            }

            if let Some(target) = self.get_best_pass_to_receiver::<T, O, F>(
                params,
                &physical,
                opponents,
                opponent_goal,
                ball,
                power,
            ) {
                let dist_to_goal = (target.x - opponent_goal_center.x).abs();
                if dist_to_goal < closest_goal {
                    closest_goal = dist_to_goal;
                    pass_target = target;
                    receiver = Some(entity);
                }
            }
        }

        (receiver, pass_target)
    }

    fn get_best_pass_to_receiver<'a, T, O, F>(
        &self,
        params: &SimulationParams,
        receiver: &PhysicalQueryItem,
        opponents: F,
        opponent_goal: &GoalQueryItem,
        ball: (&Physical, &Transform, &BoundingCircle),
        power: f32,
    ) -> Option<Vec2>
    where
        T: TeamColorMarker,
        F: Fn() -> O + Copy,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        let receiver_position = receiver.transform.translation.truncate();
        let opponent_goal_center = opponent_goal.goal.get_score_center(opponent_goal.transform);
        let ball_position = ball.1.translation.truncate();

        let time = ball
            .0
            .time_to_cover_distance(params, ball_position, receiver_position, power);
        if time < 0.0 {
            return None;
        }

        let mut intercept_range = time * receiver.physical.max_speed;

        // scale the intercept range
        let scaling_factor = 0.3;
        intercept_range *= scaling_factor;

        let (ip1, ip2) =
            get_tangent_points(receiver_position, intercept_range, ball_position).unwrap();

        let passes = [ip1, receiver_position, ip2];

        let mut closest_so_far = f32::MAX;
        let mut target = None;

        for pass in passes {
            let dist = pass.x - opponent_goal_center.x;
            if dist < closest_so_far
                && self.is_pass_safe_from_all_opponents::<T, O>(
                    params,
                    ball_position,
                    pass,
                    Some(receiver.transform),
                    opponents(),
                    (ball.0, ball.2),
                    power,
                )
            {
                closest_so_far = dist;
                target = Some(pass);
            }
        }

        target
    }

    pub fn request_pass<'a, T, O>(
        &self,
        params: &SimulationParams,
        controller: Entity,
        controller_transform: &Transform,
        receiver: Entity,
        receiver_transform: &Transform,
        opponents: O,
        ball: (&Physical, &BoundingCircle),
        player_message_dispatcher: &mut FieldPlayerMessageDispatcher,
    ) where
        T: TeamColorMarker,
        O: Iterator<Item = (PhysicalQueryItem<'a>, &'a BoundingCircle)>,
    {
        let controller_position = controller_transform.translation.truncate();
        let receiver_position = receiver_transform.translation.truncate();

        if self.is_pass_safe_from_all_opponents::<T, O>(
            params,
            controller_position,
            receiver_position,
            Some(receiver_transform),
            opponents,
            ball,
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
pub struct SoccerTeamQuery<T>
where
    T: TeamColorMarker,
{
    pub team: &'static SoccerTeam,
    pub color: &'static T,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SoccerTeamQueryMut<T>
where
    T: TeamColorMarker,
{
    pub team: &'static mut SoccerTeam,
    pub color: &'static T,

    pub state_machine: &'static mut SoccerTeamStateMachine,
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
