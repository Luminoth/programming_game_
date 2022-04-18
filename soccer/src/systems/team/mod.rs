pub mod field_player;
pub mod goalie;

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::events::team::*;
use crate::game::team::SoccerTeamState;
use crate::resources::messaging::MessageDispatcher;
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

pub fn global_state_execute(mut query: Query<SoccerTeamQueryMut>) {
    for soccer_team in query.iter_mut() {
        SoccerTeamState::execute_global(soccer_team);
    }
}

pub fn state_enter(
    mut commands: Commands,
    mut events: EventReader<SoccerTeamStateEnterEvent>,
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<SoccerTeamQueryMut>,
    receiving: Query<Entity, With<ReceivingPlayer>>,
    closest: Query<Entity, With<ClosestPlayer>>,
    controlling: Query<Entity, With<ControllingPlayer>>,
    supporting: Query<Entity, With<SupportingPlayer>>,
) {
    for event in events.iter() {
        if let Ok(team) = query.get_mut(event.entity()) {
            debug!(
                "entering soccer team state {:?} for team {:?}",
                event.state(),
                team.team.team
            );

            event.state().enter(
                &mut commands,
                &mut message_dispatcher,
                &team.team,
                receiving.get_single().ok(),
                closest.get_single().ok(),
                controlling.get_single().ok(),
                supporting.get_single().ok(),
            );
        }
    }
}

pub fn state_execute(
    mut query: Query<(Entity, SoccerTeamQueryMut)>,
    players: Query<&FieldPlayer>,
    mut exit_events: EventWriter<SoccerTeamStateExitEvent>,
    mut enter_events: EventWriter<SoccerTeamStateEnterEvent>,
) {
    for (entity, mut team) in query.iter_mut() {
        team.state_machine.current_state().execute(
            entity,
            &mut team,
            &players,
            &mut exit_events,
            &mut enter_events,
        );
    }
}
