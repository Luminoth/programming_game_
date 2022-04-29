#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::*;

pub fn GlobalState_execute<T>(
    params: Res<SimulationParams>,
    mut query: Query<(Entity, FieldPlayerQuery<T>, PhysicalQueryMut), Without<Ball>>,
    ball: Query<&Transform, With<Ball>>,
    controlling: Query<Entity, (With<T>, With<ControllingPlayer>)>,
) where
    T: TeamColorMarker,
{
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

pub fn GlobalState_on_message<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut message_events: EventReader<FieldPlayerDispatchedMessageEvent>,
    mut players: Query<(Entity, FieldPlayerQueryMut<T>, &Transform), Without<Ball>>,
    team: Query<SoccerTeamQuery<T>>,
    mut ball: Query<(&Ball, PhysicalQueryMut)>,
) where
    T: TeamColorMarker,
{
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

                    let team = team.single();
                    player.steering.target = team.team.get_best_support_spot();
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

                    ball.kick(
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

pub fn ChaseBall_enter<T>(
    mut commands: Commands,
    query: Query<(Entity, FieldPlayerQuery<T>), With<FieldPlayerStateChaseBallEnter>>,
) where
    T: TeamColorMarker,
{
    for (entity, player) in query.iter() {
        player.agent.seek_on(&mut commands, entity);
    }
}

pub fn ChaseBall_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut query: Query<
        (Entity, FieldPlayerQueryMut<T>, &Transform),
        With<FieldPlayerStateChaseBallExecute>,
    >,
    closest: Query<Entity, (With<T>, With<ClosestPlayer>)>,
    ball_physical: Query<PhysicalQuery, With<Ball>>,
) where
    T: TeamColorMarker,
{
    for (entity, mut player, transform) in query.iter_mut() {
        let ball_physical = ball_physical.single();

        // kick the ball if it's in range
        if player
            .player
            .is_ball_within_kicking_range(&params, &transform, &ball_physical.transform)
        {
            info!("kicking ball!");

            player
                .state_machine
                .change_state(&mut commands, entity, FieldPlayerState::KickBall);
            continue;
        }

        // keep chasing the ball if we're the closest to it
        if let Ok(closest) = closest.get_single() {
            if entity == closest {
                info!("chasing ball");
                player.steering.target = ball_physical.transform.translation.truncate();
                continue;
            }
        }

        info!("returning home");

        // not closest, so go home
        player.state_machine.change_state(
            &mut commands,
            entity,
            FieldPlayerState::ReturnToHomeRegion,
        );
    }
}

pub fn ChaseBall_exit<T>(
    mut commands: Commands,
    query: Query<(Entity, FieldPlayerQuery<T>), With<FieldPlayerStateChaseBallExit>>,
) where
    T: TeamColorMarker,
{
    for (entity, player) in query.iter() {
        player.agent.seek_off(&mut commands, entity);
    }
}

pub fn Wait_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    game_state: Res<GameState>,
    mut player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut query: Query<
        (
            Entity,
            FieldPlayerQueryMut<T>,
            PhysicalQueryMut,
            Option<&Arrive>,
        ),
        (With<FieldPlayerStateWaitExecute>, Without<Ball>),
    >,
    controller: Query<(Entity, &Transform), (With<T>, With<ControllingPlayer>)>,
    team: Query<SoccerTeamQuery<T>>,
    closest: Query<Entity, (With<T>, With<ClosestPlayer>)>,
    receiving: Query<Entity, (With<T>, With<ReceivingPlayer>)>,
    //players: &Query<(AnyTeamFieldPlayerQuery, PhysicalQuery)>,
    ball_physical: Query<PhysicalQuery, With<Ball>>,
    goals: Query<AnyTeamGoalQuery>,
) where
    T: TeamColorMarker,
{
    for (entity, mut player, mut physical, arrive) in query.iter_mut() {
        if !player
            .steering
            .is_at_target(&physical.transform, params.player_in_target_range_squared)
        {
            if arrive.is_none() {
                player.agent.arrive_on(&mut commands, entity);
            }
            continue;
        }

        if arrive.is_some() {
            player.agent.arrive_off(&mut commands, entity);
        }

        physical.physical.velocity = Vec2::ZERO;

        let _ball_physical = ball_physical.single();

        // TODO:
        //player.track_ball(&ball_physical.transform);
        /*
        void PlayerBase::TrackBall()
        {
          RotateHeadingToFacePosition(Ball()->Pos());
        }
        */

        if let Ok((controller, transform)) = controller.get_single() {
            if entity != controller {
                // if we're farther up the field from the controller
                // we should request a pass
                if player.player.is_ahead_of_attacker(
                    player.team,
                    &physical.transform,
                    &transform,
                    &goals,
                ) {
                    /*let team = team.single();
                    team.team.request_pass(
                        &params,
                        player.team,
                        controller,
                        transform,
                        entity,
                        physical.transform,
                        players,
                        ball_physical.physical,
                        &mut player_message_dispatcher,
                    );*/
                }
                continue;
            }
        }

        if game_state.is_game_on() {
            if let Ok(closest) = closest.get_single() {
                let have_receiver = receiving.get_single().is_ok();

                // if no one's after the ball, chase it
                if entity == closest && !have_receiver
                /*&& !goal_keeper_has_ball*/
                {
                    player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::ChaseBall,
                    );
                    continue;
                }
            }
        }
    }
}
