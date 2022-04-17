pub mod field_player;
pub mod goalie;

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::team::SoccerTeamState;
use crate::resources::SimulationParams;

pub fn update_support_spot(
    params: Res<SimulationParams>,
    mut query: Query<SupportSpotCalculatorQueryMut>,
    players: Query<(&FieldPlayer, PhysicalQuery)>,
    controller: Query<(&FieldPlayer, &Transform), With<ControllingPlayer>>,
    support: Query<(&FieldPlayer, &Transform), With<SupportingPlayer>>,
    ball: Query<BallQuery>,
    goals: Query<GoalQuery>,
) {
    if let Ok(ball) = ball.get_single() {
        if let Ok(controller) = controller.get_single() {
            for mut support_calculator in query.iter_mut() {
                // only update support spots for the controlling player's team
                if controller.0.team != support_calculator.team.team {
                    continue;
                }

                info!(
                    "updating support spot for controlling team {:?}",
                    support_calculator.team.team
                );

                let controller_position = controller.1.translation.truncate();

                let mut best_score = 0.0;
                let mut best_support_spot = None;
                for spot in &mut support_calculator.support_calculator.spots {
                    spot.score = 1.0;

                    // is it safe to pass to this spot?
                    if support_calculator.team.is_pass_safe_from_all_opponents(
                        &params,
                        controller_position,
                        spot.position,
                        None,
                        &players,
                        ball.physical,
                        params.max_passing_force,
                    ) {
                        spot.score += params.pass_safe_score;
                    }

                    // can we score a goal from this spot?
                    for goal in goals.iter() {
                        if support_calculator
                            .team
                            .can_shoot(
                                &params,
                                spot.position,
                                &goal,
                                ball.physical,
                                &players,
                                params.max_passing_force,
                            )
                            .is_some()
                        {
                            spot.score += params.can_score_score;
                        }
                    }

                    // how far away is our supporting player?
                    for support in support.iter() {
                        if support.0.team != controller.0.team {
                            continue;
                        }

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

                support_calculator.team.best_support_spot = best_support_spot;
            }
        }
    }
}

pub fn global_state_execute(mut query: Query<SoccerTeamQueryMut>) {
    for soccer_team in query.iter_mut() {
        SoccerTeamState::execute_global(soccer_team);
    }
}
