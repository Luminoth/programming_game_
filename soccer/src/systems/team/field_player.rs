#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::*;

pub fn GlobalState_execute(
    params: Res<SimulationParams>,
    mut query: Query<(Entity, FieldPlayerQuery, PhysicalQueryMut), Without<Ball>>,
    ball: Query<&Transform, With<Ball>>,
    controlling: Query<Entity, With<ControllingPlayer>>,
) {
    for (entity, player, mut physical) in query.iter_mut() {
        let ball = ball.single();
        let mut max_speed = params.player_max_speed_without_ball;

        // reduce max speed when near the ball and in possession of it
        if let Ok(controlling) = controlling.get_single() {
            if controlling == entity
                && player
                    .player
                    .is_ball_within_receiving_range(&params, &physical.transform, &ball)
            {
                max_speed = params.player_max_speed_with_ball;
            }
        }

        physical.physical.max_speed = max_speed;
    }
}

pub fn GlobalState_on_message(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut message_events: EventReader<FieldPlayerDispatchedMessageEvent>,
    mut players: Query<(Entity, FieldPlayerQueryMut, &Transform), Without<Ball>>,
    teams: Query<SoccerTeamQuery>,
    mut ball: Query<(BallQuery, PhysicalQueryMut), With<Ball>>,
) {
    for event in message_events.iter() {
        if let Ok((entity, mut player, transform)) = players.get_mut(event.receiver.unwrap()) {
            match event.message {
                FieldPlayerMessage::ReceiveBall(position) => {
                    player.steering.target = position;

                    player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::ReceiveBall,
                    );
                }
                FieldPlayerMessage::SupportAttacker => {
                    if player
                        .state_machine
                        .is_in_state(FieldPlayerState::SupportAttacker)
                    {
                        return;
                    }

                    for team in teams.iter() {
                        if team.team.team == player.player.team {
                            player.steering.target = team.team.get_best_support_spot();
                        }
                    }
                }
                FieldPlayerMessage::GoHome => {
                    player.player.home_region = player.player.default_region;

                    player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::ReturnToHomeRegion,
                    );
                }
                FieldPlayerMessage::Wait => {
                    player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::Wait,
                    );
                }
                FieldPlayerMessage::PassToMe(receiver, receiver_position) => {
                    let (ball, mut ball_physical) = ball.single_mut();
                    let ball_position = ball_physical.transform.translation.truncate();

                    if !player.player.is_ball_within_kicking_range(
                        &params,
                        transform,
                        &ball_physical.transform,
                    ) {
                        return;
                    }

                    ball.ball.kick(
                        &mut ball_physical.physical,
                        receiver_position - ball_position,
                        params.max_passing_force,
                    );

                    message_dispatcher.dispatch_message(
                        Some(receiver),
                        FieldPlayerMessage::ReceiveBall(receiver_position),
                    );
                }
            }
        }
    }
}
