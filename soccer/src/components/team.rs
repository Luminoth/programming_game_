use bevy::prelude::*;
use bevy_inspector_egui::*;
use rand::Rng;

use crate::components::goal::Goal;
use crate::components::physics::Physical;
use crate::game::team::*;
use crate::game::{BALL_RADIUS, PLAYER_RADIUS};
use crate::resources::SimulationParams;
use crate::util::point_to_world_space;

use super::state::StateMachine;

pub type SoccerTeamStateMachine = StateMachine<SoccerTeamState>;

#[derive(Debug, Default, Component, Inspectable)]
pub struct SoccerTeam {
    pub team: Team,

    pub best_support_spot: Option<Vec2>,
}

impl SoccerTeam {
    pub fn is_pass_safe_from_all_opponents(
        &self,
        params: &SimulationParams,
        from: Vec2,
        target: Vec2,
        receiver: Option<(&FieldPlayer, &Transform)>,
        players: &Query<(&FieldPlayer, &Transform, &Physical)>,
        ball_physical: &Physical,
        passing_force: f32,
    ) -> bool {
        for player in players.iter() {
            if !self.is_pass_safe_from_opponent(
                params,
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

    pub fn is_pass_safe_from_opponent(
        &self,
        params: &SimulationParams,
        from: Vec2,
        target: Vec2,
        receiver: Option<(&FieldPlayer, &Transform)>,
        opponent: (&FieldPlayer, &Transform, &Physical),
        ball_physical: &Physical,
        passing_force: f32,
    ) -> bool {
        // ignore teammates
        if opponent.0.team == self.team {
            return true;
        }

        let opponent_position = opponent.1.translation.truncate();

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
                let receiver_position = receiver.1.translation.truncate();
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
        let reach = opponent.2.max_speed * time_for_ball + BALL_RADIUS + PLAYER_RADIUS;
        local_pos_opp.y.abs() >= reach
    }

    pub fn can_shoot(
        &self,
        params: &SimulationParams,
        from: Vec2,
        goal: (&Goal, &Transform),
        ball_physical: &Physical,
        players: &Query<(&FieldPlayer, &Transform, &Physical)>,
        power: f32,
    ) -> Option<Vec2> {
        // can't score in our own goal
        if goal.0.team == self.team {
            return None;
        }

        let mut rng = rand::thread_rng();

        let goal_position = goal.1.translation.truncate();

        let mut num_attempts = params.num_attempts_to_find_valid_strike;
        while num_attempts > 0 {
            let mut target = goal_position + goal.0.score_center;

            let min_y = goal_position.y + goal.0.top.y + BALL_RADIUS;
            let max_y = goal_position.y + goal.0.bottom.y - BALL_RADIUS;

            target.y = rng.gen_range(min_y..=max_y);

            let time = ball_physical.time_to_cover_distance(params, from, target, power);
            if time >= 0.0
                && self.is_pass_safe_from_all_opponents(
                    params,
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
}

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

pub type FieldPlayerStateMachine = StateMachine<FieldPlayerState>;

#[derive(Debug, Default, Component, Inspectable)]
pub struct FieldPlayer {
    pub team: Team,
}

pub type GoalieStateMachine = StateMachine<GoalieState>;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goalie {
    pub team: Team,
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
