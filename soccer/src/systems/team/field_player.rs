#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::*;

pub fn GlobalState_execute(query: Query<FieldPlayerQuery>) {
    for _player in query.iter() {
        //debug!("executing global state for player {}", player.name.as_ref());
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
                            break;
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
