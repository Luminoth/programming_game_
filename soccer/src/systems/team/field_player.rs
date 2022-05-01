#![allow(non_snake_case)]

use bevy::prelude::*;
use rand::Rng;

use crate::components::actor::*;
use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::*;
use crate::util::*;

pub fn update<T>(time: Res<Time>, mut field_players: Query<FieldPlayerQueryMut<T>>)
where
    T: TeamColorMarker,
{
    for mut field_player in field_players.iter_mut() {
        field_player
            .field_player
            .kick_cooldown
            .tick(time.delta_seconds());
    }
}

// TODO: the functionality here makes more sense as a physics update step
// rather than being part of the state machine
pub fn GlobalState_execute<T>(
    params: Res<SimulationParams>,
    mut field_players: Query<(Entity, FieldPlayerQuery<T>, PhysicalQueryMut), Without<Ball>>,
    ball: Query<&Transform, With<Ball>>,
    controlling: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    for (entity, field_player, mut physical) in field_players.iter_mut() {
        let ball_position = ball.single().translation.truncate();

        let mut max_speed = params.player_max_speed_without_ball;

        // reduce max speed when near the ball and in possession of it
        if let Some(controlling) = controlling.optional_single() {
            if controlling.entity == entity
                && field_player.field_player.is_ball_within_receiving_range(
                    &params,
                    physical.transform,
                    ball_position,
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

                    field_player.steering.target = team.single().team.get_best_support_spot();
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
                        ball_position,
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
    closest: Query<ClosestPlayerQuery<T>>,
    ball_transform: Query<&Transform, With<Ball>>,
) where
    T: TeamColorMarker,
{
    for (entity, mut field_player, transform) in field_players.iter_mut() {
        let ball_position = ball_transform.single().translation.truncate();

        // kick the ball if it's in range
        if field_player
            .field_player
            .is_ball_within_kicking_range(&params, transform, ball_position)
        {
            info!("kicking ball!");

            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::KickBall,
            );
            continue;
        }

        // keep chasing the ball if we're the closest to it
        if let Some(closest) = closest.optional_single() {
            if entity == closest.entity {
                info!("continue chasing ball");

                field_player.steering.target = ball_position;
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
    team: Query<SoccerTeamQuery<T>>,
    controller: Query<(ControllingPlayerQuery<T>, &Transform, Option<&GoalKeeper>)>,
    closest: Query<ClosestPlayerQuery<T>>,
    receiving: Query<ReceivingPlayerQuery<T>>,
    opponents: Query<(&Actor, PhysicalQuery), (With<SoccerPlayer>, Without<T>)>,
    ball: Query<(&Actor, PhysicalQuery), With<Ball>>,
    opponent_goal: Query<&Transform, (With<Goal>, Without<T>)>,
) where
    T: TeamColorMarker,
{
    for (entity, mut field_player, mut physical, arrive) in field_players.iter_mut() {
        // get back to our home if we got bumped off it
        if !field_player
            .steering
            .is_at_target(&params, physical.transform)
        {
            if arrive.is_none() {
                info!("heading back home");

                field_player.agent.arrive_on(&mut commands, entity);
            }
            continue;
        }

        if arrive.is_some() {
            info!("arrived back home");

            field_player.agent.arrive_off(&mut commands, entity);
        }
        physical.physical.velocity = Vec2::ZERO;

        let ball = ball.single();
        let ball_position = ball.1.transform.translation.truncate();

        physical.physical.track(ball_position);

        let mut controller_is_goalkeeper = false;
        if let Some((controller, transform, goal_keeper)) = controller.optional_single() {
            controller_is_goalkeeper = goal_keeper.is_some();

            if entity != controller.entity {
                // if we're farther up the field from the controller
                // we should request a pass
                if field_player.field_player.is_ahead_of_attacker(
                    physical.transform,
                    transform,
                    opponent_goal.single(),
                ) {
                    team.single().team.request_pass::<T, _>(
                        &params,
                        controller.entity,
                        transform,
                        entity,
                        physical.transform,
                        opponents.iter(),
                        (ball.0, ball.1.physical),
                        &mut player_message_dispatcher,
                    );
                }
                continue;
            }
        }

        if game_state.is_game_on() {
            if let Some(closest) = closest.optional_single() {
                let have_receiver = receiving.optional_single().is_some();

                // if we're the closest field player
                // and no one's after the ball, chase it
                if entity == closest.entity && !have_receiver && !controller_is_goalkeeper {
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
    controlling: Query<ControllingPlayerQuery<T>>,
    receiving: Query<ReceivingPlayerQuery<T>>,
    opponents: Query<&Transform, (With<SoccerPlayer>, Without<T>)>,
    opponent_goal: Query<&Transform, (With<Goal>, Without<T>)>,
    ball: Query<Entity, With<Ball>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, field_player, transform)) = field_player.optional_single() {
        if let Some(controlling) = controlling.optional_single() {
            commands
                .entity(controlling.entity)
                .remove::<ControllingPlayer>();
        }

        if let Some(receiving) = receiving.optional_single() {
            commands
                .entity(receiving.entity)
                .remove::<ReceivingPlayer>();
        }

        let mut rng = rand::thread_rng();

        // this player is now the receiver / controller
        commands
            .entity(entity)
            .insert(ReceivingPlayer)
            .insert(ControllingPlayer);

        if field_player
            .player
            .is_in_hot_region(transform, opponent_goal.single(), &pitch)
            && rng.gen::<f32>() < params.chance_of_using_arrive_type_receive_behavior
            && !field_player.player.is_opponent_within_radius::<T, _>(
                transform,
                opponents.iter(),
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

pub fn ReceiveBall_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut field_player: Query<
        (
            Entity,
            FieldPlayerQueryMut<T>,
            PhysicalQueryMut,
            Option<&Pursuit>,
        ),
        With<FieldPlayerStateReceiveBallExecute>,
    >,
    controlling: Query<ControllingPlayerQuery<T>>,
    ball: Query<&Transform, With<Ball>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut field_player, mut physical, pursuit)) =
        field_player.optional_single_mut()
    {
        let ball_position = ball.single().translation.truncate();

        // chase the ball if it's close enough
        if field_player.field_player.is_ball_within_receiving_range(
            &params,
            physical.transform,
            ball_position,
        ) || controlling.optional_single().is_none()
        {
            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::ChaseBall,
            );
        }

        // update pursuit target
        if pursuit.is_some() {
            field_player.steering.target = ball_position;
        }

        // stop if we've arrived
        if field_player
            .steering
            .is_at_target(&params, physical.transform)
        {
            field_player.agent.arrive_off(&mut commands, entity);
            field_player.agent.pursuit_off(&mut commands, entity);

            physical.physical.track(ball_position);

            physical.physical.velocity = Vec2::ZERO;
        }
    }
}

pub fn KickBall_enter<T>(
    mut commands: Commands,
    mut field_player: Query<(Entity, FieldPlayerQueryMut<T>), With<FieldPlayerStateKickBallEnter>>,
    controlling: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut field_player)) = field_player.optional_single_mut() {
        if let Some(controlling) = controlling.optional_single() {
            commands
                .entity(controlling.entity)
                .remove::<ControllingPlayer>();
        }

        // this player is now the  controller
        commands.entity(entity).insert(ControllingPlayer);

        if !field_player.field_player.is_ready_for_next_kick() {
            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::ChaseBall,
            );
        }
    }
}

pub fn KickBall_execute<T>(
    mut commands: Commands,
    params: Res<SimulationParams>,
    mut message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut field_player: Query<
        (Entity, FieldPlayerQueryMut<T>, PhysicalQuery),
        (With<FieldPlayerStateKickBallExecute>, Without<Ball>),
    >,
    team: Query<SoccerTeamQuery<T>>,
    teammates: Query<
        (Entity, FieldPlayerQuery<T>, PhysicalQuery),
        Without<FieldPlayerStateKickBallExecute>,
    >,
    receiving: Query<ReceivingPlayerQuery<T>>,
    supporting: Query<SupportingPlayerQuery<T>>,
    controlling: Query<ControllingPlayerQuery<T>>,
    mut ball: Query<(&Ball, &Actor, PhysicalQueryMut), Without<SoccerPlayer>>,
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
    opponents: Query<(&Actor, PhysicalQuery), (With<SoccerPlayer>, Without<T>)>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut field_player, physical)) = field_player.optional_single_mut() {
        let (ball, ball_actor, mut ball_physical) = ball.single_mut();
        let ball_position = ball_physical.transform.translation.truncate();
        let position = physical.transform.translation.truncate();

        let to_ball = ball_position - position;
        let dot = physical.physical.heading.dot(to_ball.normalize_or_zero());

        // can't kick the ball if there's a receiver, or the goal keeper has it, or it's behind us
        if receiving.optional_single().is_none() || /* !goal_keeper_has_ball ||*/ dot < 0.0 {
            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::ChaseBall,
            );
            return;
        }

        let team = team.single();

        let mut rng = rand::thread_rng();

        // attempt a kick
        let power = params.max_shooting_force * dot;
        let (mut ball_target, can_shoot) = team.team.can_shoot::<T, _, _>(
            &params,
            ball_position,
            opponent_goal.single(),
            (ball_actor, &ball_physical.physical),
            || opponents.iter(),
            power,
        );
        if can_shoot || rng.gen::<f32>() < params.chance_player_attempts_pot_shot {
            ball_target = ball.add_noise_to_kick(&params, ball_physical.transform, ball_target);
            let direction = ball_target - ball_position;
            ball.kick(&mut ball_physical.physical, direction, power);

            field_player
                .state_machine
                .change_state(&mut commands, entity, FieldPlayerState::Wait);

            field_player.player.find_support(
                &mut commands,
                &mut message_dispatcher,
                &team.team,
                teammates.iter(),
                supporting.optional_single().map(|x| x.entity),
                controlling.single().entity,
            );
            return;
        }

        // can't kick, attempt a pass
        // TODO:
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
    closest: Query<ClosestPlayerQuery<T>>,
    controlling_goal_keeper: Query<ControllingPlayerQuery<T>, With<GoalKeeper>>,
    receiving: Query<ReceivingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    for (entity, mut field_player, transform) in field_players.iter_mut() {
        if game_state.is_game_on() {
            if let Some(closest) = closest.optional_single() {
                let have_receiver = receiving.optional_single().is_some();

                // if we're the closest field player
                // and no one's after the ball, chase it
                if entity == closest.entity
                    && !have_receiver
                    && controlling_goal_keeper.optional_single().is_none()
                {
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
        } else if field_player.steering.is_at_target(&params, transform) {
            field_player
                .state_machine
                .change_state(&mut commands, entity, FieldPlayerState::Wait);
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
