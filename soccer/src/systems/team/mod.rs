#![allow(non_snake_case)]

pub mod field_player;
pub mod goalie;

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::messaging::MessageEvent;
use crate::resources::messaging::MessageDispatcher;
use crate::resources::pitch::*;
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
    if let Ok(controller) = controller.get_single() {
        let ball = ball.single();

        for mut support_calculator in query.iter_mut() {
            // only update support spots for the controlling player's team
            if controller.0.team != support_calculator.team.team {
                continue;
            }

            info!(
                "updating support spot for controlling team {:?}",
                support_calculator.team.team
            );

            support_calculator.team.best_support_spot = None;

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
                if let Ok(support) = support.get_single() {
                    assert!(support.0.team == controller.0.team);

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

pub fn GlobalState_execute(query: Query<SoccerTeamQuery>) {
    for soccer_team in query.iter() {
        debug!(
            "executing global state for team {:?}",
            soccer_team.team.team
        );
    }
}

pub fn PrepareForKickOff_enter(
    mut commands: Commands,
    mut message_dispatcher: ResMut<MessageDispatcher>,
    query: Query<SoccerTeamQuery, With<SoccerTeamStatePrepareForKickOffEnter>>,
    receiving: Query<Entity, With<ReceivingPlayer>>,
    closest: Query<Entity, With<ClosestPlayer>>,
    controlling: Query<Entity, With<ControllingPlayer>>,
    supporting: Query<Entity, With<SupportingPlayer>>,
) {
    for team in query.iter() {
        info!("{:?} team preparing for kick off", team.team.team);

        // reset player positions

        if let Ok(receiving) = receiving.get_single() {
            commands.entity(receiving).remove::<ReceivingPlayer>();
        }

        if let Ok(closest) = closest.get_single() {
            commands.entity(closest).remove::<ClosestPlayer>();
        }

        if let Ok(controlling) = controlling.get_single() {
            commands.entity(controlling).remove::<ControllingPlayer>();
        }

        if let Ok(supporting) = supporting.get_single() {
            commands.entity(supporting).remove::<SupportingPlayer>();
        }

        // send players home
        message_dispatcher.dispatch_message(None, MessageEvent::GoHome(team.team.team));
    }
}

pub fn PrepareForKickOff_execute(
    mut commands: Commands,
    pitch: Res<Pitch>,
    mut query: Query<(Entity, SoccerTeamQueryMut), With<SoccerTeamStatePrepareForKickOffExecute>>,
    players: Query<(&FieldPlayer, &Transform)>,
    goalies: Query<(&Goalie, &Transform)>,
) {
    info!("Waiting for teams ready ...");

    for (player, transform) in players.iter() {
        if !player.is_in_home_region(transform, &pitch) {
            return;
        }
    }

    for (goalie, transform) in goalies.iter() {
        if !goalie.is_in_home_region(transform, &pitch) {
            return;
        }
    }

    for (entity, mut team) in query.iter_mut() {
        team.state_machine
            .change_state(&mut commands, entity, SoccerTeamState::Defending);
    }
}

pub fn Defending_enter(query: Query<SoccerTeamQuery, With<SoccerTeamStateDefendingEnter>>) {
    for team in query.iter() {
        info!("{:?} team preparing for kick off", team.team.team);

        // TODO:
    }
}
