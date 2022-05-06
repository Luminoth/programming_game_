#![allow(non_snake_case)]

use bevy::prelude::*;
use rand::Rng;

use crate::components::actor::*;
use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::events::*;
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

pub fn update_physics<T>(mut field_players: Query<(FieldPlayerQuery<T>, PhysicalQueryMut)>)
where
    T: TeamColorMarker,
{
    for (field_player, mut physical) in field_players.iter_mut() {
        // if no steering force is produced, decelerate the player
        if field_player.steering.accumulated_force == Vec2::ZERO {
            let braking_rate = 0.02;
            physical.physical.velocity *= braking_rate;
        }
    }
}

pub fn find_support_event_handler<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut events: EventReader<FindSupportEvent>,
    players: Query<&SoccerPlayer, With<T>>,
    mut team: Query<(SoccerTeamQueryMut<T>, &mut SupportSpotCalculator)>,
    teammates: Query<(Entity, FieldPlayerQuery<T>, PhysicalQuery)>,
    supporting: Query<SupportingPlayerQuery<T>>,
    controlling: Query<(ControllingPlayerQuery<T>, &Transform)>,
    opponents: Query<(&Actor, PhysicalQuery), (With<SoccerPlayer>, Without<T>)>,
    ball: Query<(&Actor, &Physical), With<Ball>>,
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    let (mut team, mut support_calculator) = team.single_mut();

    // TODO: if more than one event is directed
    // at a single player then this will over-call find_support()
    // we should do something to squash events so we only
    // make the call once per-player
    for event in events.iter() {
        if let Ok(player) = players.get(event.0) {
            let controlling = controlling.single();

            player.find_support(
                &mut commands,
                &params,
                &mut message_dispatcher,
                &mut team,
                &mut support_calculator,
                teammates.iter(),
                || opponents.iter(),
                supporting.optional_single().map(|x| x.entity),
                (controlling.0.entity, controlling.1),
                ball.single(),
                opponent_goal.single(),
            );
        }
    }
}

// TODO: the functionality here makes more sense as a physics update step
// rather than being part of the state machine
pub fn GlobalState_execute<T>(
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut field_players: Query<(Entity, FieldPlayerQuery<T>, PhysicalQueryMut), Without<Ball>>,
    ball: Query<&Transform, With<Ball>>,
    controlling: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    let ball_position = ball.single().translation.truncate();

    for (entity, field_player, mut physical) in field_players.iter_mut() {
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
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut message_events: EventReader<FieldPlayerDispatchedMessageEvent>,
    mut find_support_events: EventWriter<FindSupportEvent>,
    mut field_players: Query<(Entity, FieldPlayerQueryMut<T>, &Transform), Without<Ball>>,
    team: Query<SoccerTeamQuery<T>>,
    receiving: Query<ReceivingPlayerQuery<T>>,
    mut ball: Query<(&Ball, PhysicalQueryMut)>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    let team = team.single();

    let (ball, mut ball_physical) = ball.single_mut();
    let ball_position = ball_physical.transform.translation.truncate();

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

                    field_player.steering.target = team.team.best_support_spot.unwrap();

                    field_player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::SupportAttacker,
                    );
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
                    info!(
                        "{} received request from {:?} to make pass",
                        field_player.name, receiver
                    );

                    // if the ball is not within range
                    // or there is already a receiver
                    // then the player cannot pass the ball
                    if receiving.optional_single().is_some()
                        || !field_player.field_player.is_ball_within_kicking_range(
                            &params,
                            transform,
                            ball_position,
                        )
                    {
                        warn!(
                            "{} cannot make request pass <cannot kick ball>",
                            field_player.name
                        );
                        return;
                    }

                    ball.kick(
                        &mut ball_physical.physical,
                        receiver_position - ball_position,
                        params.max_passing_force,
                    );

                    info!(
                        "{} passed ball to requesting player {:?}",
                        field_player.name, receiver
                    );

                    // let the receiver know the pass is incoming
                    message_dispatcher.dispatch_message(
                        Some(receiver),
                        FieldPlayerMessage::ReceiveBall(receiver_position),
                    );

                    field_player.state_machine.change_state(
                        &mut commands,
                        entity,
                        FieldPlayerState::Wait,
                    );

                    find_support_events.send(FindSupportEvent(entity));
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
        info!("{} enters chase state", field_player.name);

        field_player.agent.seek_on(&mut commands, entity);
    }
}

pub fn ChaseBall_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut field_players: Query<
        (Entity, FieldPlayerQueryMut<T>, &Transform),
        With<FieldPlayerStateChaseBallExecute>,
    >,
    closest: Query<ClosestPlayerQuery<T>>,
    ball_transform: Query<&Transform, With<Ball>>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    let ball_position = ball_transform.single().translation.truncate();

    for (entity, mut field_player, transform) in field_players.iter_mut() {
        // kick the ball if it's in range
        if field_player
            .field_player
            .is_ball_within_kicking_range(&params, transform, ball_position)
        {
            info!("transitioning from chasing to kicking ball state!");

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

        info!("lost the ball while chasing, transitioning to return home state");

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

pub fn Wait_enter<T>(
    game_state: Res<GameState>,
    pitch: Res<Pitch>,
    mut field_players: Query<FieldPlayerQueryMut<T>, With<FieldPlayerStateWaitEnter>>,
) where
    T: TeamColorMarker,
{
    for mut field_player in field_players.iter_mut() {
        info!("{} enters wait state", field_player.name);

        if !game_state.is_game_on() {
            field_player.steering.target = field_player.player.get_home_region(&pitch).position;
        }
    }
}

pub fn Wait_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
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
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    let ball = ball.single();
    let ball_position = ball.1.transform.translation.truncate();

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

        physical.physical.track(ball_position);

        let mut controller_is_goalkeeper = false;
        if let Some((controller, controller_transform, goal_keeper)) = controller.optional_single()
        {
            controller_is_goalkeeper = goal_keeper.is_some();

            // if we're not the controller
            // and we're farther up the field from the controller
            // we should request a pass
            if entity != controller.entity
                && field_player.field_player.is_ahead_of_attacker(
                    physical.transform,
                    controller_transform,
                    opponent_goal.single(),
                )
            {
                team.single().team.request_pass::<T, _>(
                    &params,
                    controller.entity,
                    controller_transform,
                    entity,
                    physical.transform,
                    opponents.iter(),
                    (ball.0, ball.1.physical),
                    &mut player_message_dispatcher,
                );
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
    params_asset: Res<SimulationParamsAsset>,
    params_assets: Res<Assets<SimulationParams>>,
    pitch: Res<Pitch>,
    field_player: Query<
        (Entity, FieldPlayerQuery<T>, &Transform),
        With<FieldPlayerStateReceiveBallEnter>,
    >,
    controlling: Query<ControllingPlayerQuery<T>>,
    receiving: Query<ReceivingPlayerQuery<T>>,
    opponents: Query<&Transform, (With<SoccerPlayer>, Without<T>)>,
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
    ball: Query<Entity, With<Ball>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, field_player, transform)) = field_player.optional_single() {
        let params = params_assets.get(&params_asset.handle).unwrap();

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
            info!("{} enters receive state (using arrive)", field_player.name);
            field_player.agent.arrive_on(&mut commands, entity);
        } else {
            info!("{} enters receive state (using pursuit)", field_player.name);
            field_player
                .agent
                .pursuit_on(&mut commands, entity, ball.single());
        }
    }
}

pub fn ReceiveBall_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: Res<Assets<SimulationParams>>,
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
        let params = params_assets.get(&params_asset.handle).unwrap();

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
            return;
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

pub fn ReceiveBall_exit<T>(
    mut commands: Commands,
    field_player: Query<(Entity, FieldPlayerQuery<T>), With<FieldPlayerStateReceiveBallExit>>,
    receiving: Query<ReceivingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, field_player)) = field_player.optional_single() {
        field_player.agent.arrive_off(&mut commands, entity);
        field_player.agent.pursuit_off(&mut commands, entity);

        if let Some(receiving) = receiving.optional_single() {
            commands
                .entity(receiving.entity)
                .remove::<ReceivingPlayer>();
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
            warn!("kick ball on cooldown!");

            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::ChaseBall,
            );
        }

        info!("{} enters kick state", field_player.name);
    }
}

pub fn KickBall_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: Res<Assets<SimulationParams>>,
    mut message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut find_support_events: EventWriter<FindSupportEvent>,
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
    controlling_goal_keeper: Query<ControllingPlayerQuery<T>, With<GoalKeeper>>,
    mut ball: Query<(&Ball, &Actor, PhysicalQueryMut), Without<SoccerPlayer>>,
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
    opponents: Query<(&Actor, PhysicalQuery), (With<SoccerPlayer>, Without<T>)>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut field_player, physical)) = field_player.optional_single_mut() {
        let params = params_assets.get(&params_asset.handle).unwrap();

        let (ball, ball_actor, mut ball_physical) = ball.single_mut();
        let ball_position = ball_physical.transform.translation.truncate();
        let position = physical.transform.translation.truncate();

        let to_ball = ball_position - position;
        let dot = physical.physical.heading.dot(to_ball.normalize_or_zero());

        let have_receiver = receiving.optional_single().is_some();
        let controller_is_goalkeeper = controlling_goal_keeper.optional_single().is_some();

        // can't kick the ball if there's a receiver, or the goal keeper has it, or it's behind us
        if have_receiver || controller_is_goalkeeper || dot < -field_player.actor.bounding_radius {
            info!(
                "have a receiver already ({}) / goalie has ball ({}) / ball behind player ({})",
                have_receiver, controller_is_goalkeeper, dot
            );

            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::ChaseBall,
            );
            return;
        }

        let team = team.single();
        let opponent_goal = opponent_goal.single();

        let mut rng = rand::thread_rng();

        // attempt a kick
        let power = params.max_shooting_force * dot;
        let (mut ball_target, can_shoot) = team.team.can_shoot::<T, _, _>(
            &params,
            ball_position,
            opponent_goal,
            (ball_actor, &ball_physical.physical),
            || opponents.iter(),
            power,
        );
        if can_shoot || rng.gen::<f32>() < params.chance_player_attempts_pot_shot {
            info!("{} attempts a shot at {}", field_player.name, ball_target);

            ball_target = ball.add_noise_to_kick(&params, ball_physical.transform, ball_target);
            let direction = ball_target - ball_position;
            ball.kick(&mut ball_physical.physical, direction, power);

            field_player
                .state_machine
                .change_state(&mut commands, entity, FieldPlayerState::Wait);

            find_support_events.send(FindSupportEvent(entity));
            return;
        }

        // can't kick, attempt a pass
        let power = params.max_passing_force * dot;
        if field_player.player.is_threatened(
            &params,
            physical.transform,
            physical.physical,
            opponents.iter(),
        ) {
            let (receiver, mut ball_target) = team.team.can_pass::<T, _, _, _>(
                &params,
                (entity, physical.transform),
                teammates.iter(),
                || opponents.iter(),
                opponent_goal,
                (ball_actor, &ball_physical.physical, ball_physical.transform),
                power,
                params.min_pass_distance,
            );
            if let Some(receiver) = receiver {
                ball_target = ball.add_noise_to_kick(&params, ball_physical.transform, ball_target);
                let direction = ball_target - ball_position;
                ball.kick(&mut ball_physical.physical, direction, power);

                info!(
                    "{} passes the ball with force {} to player {:?} target is {}",
                    field_player.name, power, receiver, ball_target
                );

                message_dispatcher
                    .dispatch_message(Some(receiver), FieldPlayerMessage::ReceiveBall(ball_target));

                field_player.state_machine.change_state(
                    &mut commands,
                    entity,
                    FieldPlayerState::Wait,
                );

                find_support_events.send(FindSupportEvent(entity));
                return;
            }
        }

        // can't shoot or pass, so dribble up the field
        field_player
            .state_machine
            .change_state(&mut commands, entity, FieldPlayerState::Dribble);

        find_support_events.send(FindSupportEvent(entity));
    }
}

pub fn Dribble_enter<T>(
    mut commands: Commands,
    field_player: Query<(Entity, FieldPlayerQuery<T>), With<FieldPlayerStateDribbleEnter>>,
    controlling: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, field_player)) = field_player.optional_single() {
        if let Some(controlling) = controlling.optional_single() {
            commands
                .entity(controlling.entity)
                .remove::<ControllingPlayer>();
        }

        // this player is now the controller
        commands.entity(entity).insert(ControllingPlayer);

        info!("{} enters dribble state", field_player.name);
    }
}

pub fn Dribble_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: Res<Assets<SimulationParams>>,
    mut field_player: Query<
        (Entity, FieldPlayerQueryMut<T>, PhysicalQuery),
        With<FieldPlayerStateDribbleExecute>,
    >,
    goal: Query<(&Goal, &Transform), With<T>>,
    mut ball: Query<(&Ball, PhysicalQueryMut), Without<SoccerPlayer>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut field_player, physical)) = field_player.optional_single_mut() {
        let params = params_assets.get(&params_asset.handle).unwrap();

        let goal = goal.single();
        let (ball, mut ball_physical) = ball.single_mut();

        // if the ball is between the player and their own goal
        // then we have to bring the ball around to the other side
        let dot = goal.0.facing.dot(physical.physical.heading);
        if dot < -field_player.actor.bounding_radius {
            let angle =
                -std::f32::consts::FRAC_PI_4 * goal.0.facing.sign(physical.physical.heading);
            let direction = rotate_around_origin(physical.physical.heading, angle);

            let kicking_force = 0.8;
            ball.kick(&mut ball_physical.physical, direction, kicking_force);
        } else {
            ball.kick(
                &mut ball_physical.physical,
                goal.0.facing,
                params.max_dribble_force,
            );
        }

        field_player
            .state_machine
            .change_state(&mut commands, entity, FieldPlayerState::ChaseBall);
    }
}

pub fn SupportAttacker_enter<T>(
    mut commands: Commands,
    mut field_player: Query<
        (Entity, FieldPlayerQueryMut<T>),
        With<FieldPlayerStateSupportAttackerEnter>,
    >,
    team: Query<SoccerTeamQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut field_player)) = field_player.optional_single_mut() {
        let team = team.single();

        field_player.agent.arrive_on(&mut commands, entity);

        field_player.steering.target = team.team.best_support_spot.unwrap();

        info!("{} enters support state", field_player.name);
    }
}

pub fn SupportAttacker_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: Res<Assets<SimulationParams>>,
    mut player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut field_player: Query<
        (Entity, FieldPlayerQueryMut<T>, PhysicalQueryMut),
        With<FieldPlayerStateSupportAttackerExecute>,
    >,
    controller: Query<(ControllingPlayerQuery<T>, &Transform)>,
    team: Query<SoccerTeamQuery<T>>,
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
    opponents: Query<(&Actor, PhysicalQuery), (With<SoccerPlayer>, Without<T>)>,
    ball: Query<(&Actor, PhysicalQuery), (With<Ball>, Without<SoccerPlayer>)>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut field_player, mut physical)) = field_player.optional_single_mut() {
        let params = params_assets.get(&params_asset.handle).unwrap();

        // if we lost control, go back home
        if controller.optional_single().is_none() {
            field_player.state_machine.change_state(
                &mut commands,
                entity,
                FieldPlayerState::ReturnToHomeRegion,
            );
            return;
        }
        let (controller, controller_transform) = controller.single();

        let team = team.single();

        // if the support target changed, move to the new location
        let best_supporting_spot = team.team.best_support_spot.unwrap();
        if field_player.steering.target != best_supporting_spot {
            field_player.steering.target = best_supporting_spot;
            field_player.agent.arrive_on(&mut commands, entity);
        }

        let (ball_actor, ball_physical) = ball.single();
        let ball_position = ball_physical.transform.translation.truncate();

        let opponent_goal = opponent_goal.single();

        // if we can shoot, request a pass
        let (_, can_shoot) = team.team.can_shoot::<T, _, _>(
            &params,
            ball_position,
            opponent_goal,
            (ball_actor, &ball_physical.physical),
            || opponents.iter(),
            params.max_shooting_force,
        );
        if can_shoot {
            team.team.request_pass::<T, _>(
                &params,
                controller.entity,
                controller_transform,
                entity,
                physical.transform,
                opponents.iter(),
                (ball_actor, ball_physical.physical),
                &mut player_message_dispatcher,
            );
        }

        if field_player
            .steering
            .is_at_target(&params, physical.transform)
        {
            field_player.agent.arrive_off(&mut commands, entity);

            physical.physical.track(ball_position);

            physical.physical.velocity = Vec2::ZERO;

            // if we're not threatened by another player,
            // and didn't already request one, request a pass
            if !can_shoot
                && !field_player.player.is_threatened(
                    &params,
                    physical.transform,
                    &physical.physical,
                    opponents.iter(),
                )
            {
                team.team.request_pass::<T, _>(
                    &params,
                    controller.entity,
                    controller_transform,
                    entity,
                    physical.transform,
                    opponents.iter(),
                    (ball_actor, ball_physical.physical),
                    &mut player_message_dispatcher,
                );
            }
        }
    }
}

pub fn SupportAttacker_exit<T>(
    mut commands: Commands,
    field_player: Query<(Entity, FieldPlayerQuery<T>), With<FieldPlayerStateSupportAttackerExit>>,
    supporting: Query<SupportingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, field_player)) = field_player.optional_single() {
        if let Some(supporting) = supporting.optional_single() {
            commands
                .entity(supporting.entity)
                .remove::<SupportingPlayer>();
        }

        field_player.agent.arrive_off(&mut commands, entity);
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

        info!("{} enters ReturnToHome state", field_player.name);
    }
}

pub fn ReturnToHomeRegion_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: Res<Assets<SimulationParams>>,
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
    let params = params_assets.get(&params_asset.handle).unwrap();

    for (entity, mut field_player, transform) in field_players.iter_mut() {
        if game_state.is_game_on() {
            if let Some(closest) = closest.optional_single() {
                let have_receiver = receiving.optional_single().is_some();
                let controller_is_goalkeeper = controlling_goal_keeper.optional_single().is_some();

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
