#![allow(non_snake_case)]

use bevy::prelude::*;
use rand::Rng;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::*;

// TODO: the functionality here makes more sense as a physics update step
// rather than being part of the state machine
pub fn GlobalState_execute<T>(
    params: Res<SimulationParams>,
    mut field_players: Query<(Entity, FieldPlayerQuery<T>, PhysicalQueryMut), Without<Ball>>,
    ball: Query<&Transform, With<Ball>>,
    controlling: Query<Entity, (With<T>, With<ControllingPlayer>)>,
) where
    T: TeamColorMarker,
{
    for (entity, field_player, mut physical) in field_players.iter_mut() {
        let ball = ball.single();

        let mut max_speed = params.player_max_speed_without_ball;

        // reduce max speed when near the ball and in possession of it
        if let Ok(controlling) = controlling.get_single() {
            if controlling == entity
                && field_player.field_player.is_ball_within_receiving_range(
                    &params,
                    &physical.transform,
                    &ball,
                )
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
    mut field_players: Query<(Entity, FieldPlayerQueryMut<T>, &Transform), Without<Ball>>,
    team: Query<SoccerTeamQuery<T>>,
    mut ball: Query<(&Ball, PhysicalQueryMut)>,
) where
    T: TeamColorMarker,
{
    for event in message_events.iter() {
        if let Ok((entity, mut field_player, transform)) =
            field_players.get_mut(event.receiver.unwrap())
        {
            match event.message {
                FieldPlayerMessage::ReceiveBall(position) => {
                    field_player.steering.target = position;

                    field_player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::ReceiveBall,
                    );
                }
                FieldPlayerMessage::SupportAttacker => {
                    if field_player
                        .state_machine
                        .is_in_state(FieldPlayerState::SupportAttacker)
                    {
                        return;
                    }

                    let team = team.single();
                    field_player.steering.target = team.team.get_best_support_spot();
                }
                FieldPlayerMessage::GoHome => {
                    field_player.player.home_region = field_player.player.default_region;

                    field_player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::ReturnToHomeRegion,
                    );
                }
                FieldPlayerMessage::Wait => {
                    field_player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::Wait,
                    );
                }
                FieldPlayerMessage::PassToMe(receiver, receiver_position) => {
                    let (ball, mut ball_physical) = ball.single_mut();
                    let ball_position = ball_physical.transform.translation.truncate();

                    if !field_player.field_player.is_ball_within_kicking_range(
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
    field_players: Query<(Entity, FieldPlayerQuery<T>), With<FieldPlayerStateChaseBallEnter>>,
) where
    T: TeamColorMarker,
{
    for (entity, field_player) in field_players.iter() {
        field_player.agent.seek_on(&mut commands, entity);
    }
}

pub fn ChaseBall_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut field_players: Query<
        (Entity, FieldPlayerQueryMut<T>, &Transform),
        With<FieldPlayerStateChaseBallExecute>,
    >,
    closest: Query<Entity, (With<T>, With<ClosestPlayer>)>,
    ball_physical: Query<PhysicalQuery, With<Ball>>,
) where
    T: TeamColorMarker,
{
    for (entity, mut field_player, transform) in field_players.iter_mut() {
        let ball_physical = ball_physical.single();

        // kick the ball if it's in range
        if field_player.field_player.is_ball_within_kicking_range(
            &params,
            &transform,
            &ball_physical.transform,
        ) {
            info!("kicking ball!");

            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::KickBall,
            );
            continue;
        }

        // keep chasing the ball if we're the closest to it
        if let Ok(closest) = closest.get_single() {
            if entity == closest {
                info!("continue chasing ball");

                field_player.steering.target = ball_physical.transform.translation.truncate();
                continue;
            }
        }

        info!("lost the ball, returning home");

        // not closest, so go home
        field_player.state_machine.change_state(
            &mut commands,
            entity,
            FieldPlayerState::ReturnToHomeRegion,
        );
    }
}

pub fn ChaseBall_exit<T>(
    mut commands: Commands,
    field_players: Query<(Entity, FieldPlayerQuery<T>), With<FieldPlayerStateChaseBallExit>>,
) where
    T: TeamColorMarker,
{
    for (entity, field_player) in field_players.iter() {
        field_player.agent.seek_off(&mut commands, entity);
    }
}

pub fn Wait_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    game_state: Res<GameState>,
    mut player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut field_players: Query<
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
    closest: Query<Entity, (With<FieldPlayer>, With<T>, With<ClosestPlayer>)>,
    receiving: Query<Entity, (With<T>, With<ReceivingPlayer>)>,
    //players: &Query<(AnyTeamSoccerPlayerQuery, PhysicalQuery)>,
    ball_physical: Query<PhysicalQuery, With<Ball>>,
    goals: Query<AnyTeamGoalQuery>,
) where
    T: TeamColorMarker,
{
    for (entity, mut field_player, mut physical, arrive) in field_players.iter_mut() {
        // get back to our home if we got bumped off it
        if !field_player
            .steering
            .is_at_target(&physical.transform, params.player_in_target_range_squared)
        {
            if arrive.is_none() {
                field_player.agent.arrive_on(&mut commands, entity);
            }
            continue;
        }

        if arrive.is_some() {
            field_player.agent.arrive_off(&mut commands, entity);
        }
        physical.physical.velocity = Vec2::ZERO;

        //let ball_physical = ball_physical.single();

        // TODO:
        //field_player.player.track_ball(&ball_physical.transform);
        /*void PlayerBase::TrackBall()
        {
          RotateHeadingToFacePosition(Ball()->Pos());
        }
        */

        if let Ok((controller, transform)) = controller.get_single() {
            if entity != controller {
                // if we're farther up the field from the controller
                // we should request a pass
                if field_player.field_player.is_ahead_of_attacker(
                    field_player.team,
                    &physical.transform,
                    &transform,
                    &goals,
                ) {
                    /*let team = team.single();
                    team.team.request_pass(
                        &params,
                        field_player.team,
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

                // if we're the closest field player
                // and no one's after the ball, chase it
                if entity == closest && !have_receiver {
                    field_player.state_machine.change_state(
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

pub fn ReceiveBall_enter<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    pitch: Res<Pitch>,
    field_player: Query<
        (Entity, FieldPlayerQuery<T>, &Transform),
        With<FieldPlayerStateReceiveBallEnter>,
    >,
    controlling: Query<Entity, (With<T>, With<ControllingPlayer>)>,
    receiving: Query<Entity, (With<T>, With<ReceivingPlayer>)>,
    players: Query<(AnyTeamSoccerPlayerQuery, PhysicalQuery)>,
    goals: Query<AnyTeamGoalQuery>,
    ball: Query<Entity, With<Ball>>,
) where
    T: TeamColorMarker,
{
    if let Ok(controlling) = controlling.get_single() {
        commands.entity(controlling).remove::<ControllingPlayer>();
    }

    if let Ok(receiving) = receiving.get_single() {
        commands.entity(receiving).remove::<ReceivingPlayer>();
    }

    let mut rng = rand::thread_rng();

    if let Ok((entity, field_player, transform)) = field_player.get_single() {
        // this player is now the receiver / controller
        commands
            .entity(entity)
            .insert(ReceivingPlayer)
            .insert(ControllingPlayer);

        if field_player
            .player
            .is_in_hot_region(field_player.team, transform, &goals, &pitch)
            && rng.gen::<f32>() < params.chance_of_using_arrive_type_receive_behavior
            && !field_player.player.is_opponent_within_radius(
                field_player.team,
                transform,
                &players,
                params.pass_threat_radius,
            )
        {
            field_player.agent.arrive_on(&mut commands, entity);
        } else {
            field_player
                .agent
                .pursuit_on(&mut commands, entity, ball.single());
        }
    }
}

pub fn ReturnToHomeRegion_enter<T>(
    mut commands: Commands,
    pitch: Res<Pitch>,
    mut field_players: Query<
        (Entity, FieldPlayerQueryMut<T>),
        With<FieldPlayerStateReturnToHomeRegionEnter>,
    >,
) where
    T: TeamColorMarker,
{
    for (entity, mut field_player) in field_players.iter_mut() {
        field_player.agent.arrive_on(&mut commands, entity);

        if !field_player
            .player
            .get_home_region(&pitch)
            .is_inside_half(field_player.steering.target)
        {
            field_player.steering.target = field_player.player.get_home_region(&pitch).position;
        }
    }
}

pub fn ReturnToHomeRegion_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    game_state: Res<GameState>,
    pitch: Res<Pitch>,
    mut field_players: Query<
        (Entity, FieldPlayerQueryMut<T>, &Transform),
        With<FieldPlayerStateReturnToHomeRegionExecute>,
    >,
    closest: Query<Entity, (With<FieldPlayer>, With<T>, With<ClosestPlayer>)>,
    receiving: Query<Entity, (With<T>, With<ReceivingPlayer>)>,
) where
    T: TeamColorMarker,
{
    for (entity, mut field_player, transform) in field_players.iter_mut() {
        if game_state.is_game_on() {
            if let Ok(closest) = closest.get_single() {
                let have_receiver = receiving.get_single().is_ok();

                // if we're the closest field player
                // and no one's after the ball, chase it
                if entity == closest && !have_receiver {
                    field_player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::ChaseBall,
                    );
                    continue;
                }
            }

            let position = transform.translation.truncate();
            if field_player
                .player
                .get_home_region(&pitch)
                .is_inside_half(position)
            {
                field_player.steering.target = position;
                field_player.state_machine.change_state(
                    &mut commands,
                    entity,
                    FieldPlayerState::Wait,
                );
            }
        } else {
            if field_player
                .steering
                .is_at_target(transform, params.player_in_target_range_squared)
            {
                field_player.state_machine.change_state(
                    &mut commands,
                    entity,
                    FieldPlayerState::Wait,
                );
            }
        }
    }
}

pub fn ReturnToHomeRegion_exit<T>(
    mut commands: Commands,
    field_players: Query<
        (Entity, FieldPlayerQuery<T>),
        With<FieldPlayerStateReturnToHomeRegionExit>,
    >,
) where
    T: TeamColorMarker,
{
    for (entity, field_player) in field_players.iter() {
        field_player.agent.arrive_off(&mut commands, entity);
    }
}
