#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::*;
use crate::util::*;

pub fn update<T>(
    mut goal_keeper: Query<(GoalKeeperQuery<T>, PhysicalQueryMut)>,
    controller: Query<&ControllingPlayer, (With<T>, With<GoalKeeper>)>,
    ball_transform: Query<&Transform, With<Ball>>,
) where
    T: TeamColorMarker,
{
    let ball_transform = ball_transform.single();

    if let Some((_, mut physical)) = goal_keeper.optional_single_mut() {
        // track the ball if we aren't controlling it
        if controller.optional_single().is_none() {
            physical
                .physical
                .track(physical.transform, ball_transform.translation.truncate());
        }
    }
}

pub fn GlobalState_on_message<T>(
    mut _commands: Commands,
    mut message_events: EventReader<GoalKeeperDispatchedMessageEvent>,
    mut goal_keeper: Query<(Entity, GoalKeeperQueryMut<T>)>,
) where
    T: TeamColorMarker,
{
    for event in message_events.iter() {
        if let Some((entity, mut _goal_keeper)) = goal_keeper.optional_single_mut() {
            if entity != event.receiver.unwrap() {
                continue;
            }

            match event.message {
                /*GoalKeeperMessage::GoHome => {
                    goal_keeper.player.home_region = goal_keeper.player.default_region;

                    goal_keeper.state_machine.change_state(
                        &mut commands,
                        entity,
                        GoalKeeperState::ReturnHome,
                    );
                }*/
                /*GoalKeeperMessage::ReceiveBall => {
                    goal_keeper.state_machine.change_state(
                        &mut commands,
                        entity,
                        GoalKeeperState::InterceptBall,
                    );
                }*/
            }
        }
    }
}

pub fn TendGoal_enter<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut goal_keeper: Query<(Entity, GoalKeeperQueryMut<T>), With<GoalKeeperStateTendGoalEnter>>,
    goal: Query<TeamGoalQuery<T>>,
    ball: Query<(Entity, &Transform), With<Ball>>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    if let Some((entity, mut goal_keeper)) = goal_keeper.optional_single_mut() {
        let goal = goal.single();
        let (ball, ball_transform) = ball.single();

        goal_keeper.agent.interpose_on(
            &mut commands,
            entity,
            ball,
            params.goal_keeper_tending_distance,
        );

        goal_keeper.steering.target =
            goal_keeper
                .goal_keeper
                .get_rear_interpose_target(params, &goal, ball_transform);

        info!("{} enters tend goal state", goal_keeper.name);
    }
}

pub fn TendGoal_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut goal_keeper: Query<
        (Entity, GoalKeeperQueryMut<T>, PhysicalQuery),
        (With<GoalKeeperStateTendGoalExecute>, Without<Ball>),
    >,
    goal: Query<TeamGoalQuery<T>>,
    mut ball: Query<PhysicalQueryMut, With<Ball>>,
    controlling: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    if let Some((entity, mut goal_keeper, physical)) = goal_keeper.optional_single_mut() {
        let mut ball_physical = ball.single_mut();
        let goal = goal.single();

        // update interpose target as the ball moves
        goal_keeper.steering.target = goal_keeper.goal_keeper.get_rear_interpose_target(
            params,
            &goal,
            ball_physical.transform,
        );

        // if the ball comes in range, trap it and put it back in play
        if goal_keeper.goal_keeper.is_ball_within_keeper_range(
            params,
            physical.transform,
            ball_physical.transform,
        ) {
            ball_physical.physical.velocity = Vec2::ZERO;

            if let Some(controlling) = controlling.optional_single() {
                commands
                    .entity(controlling.entity)
                    .remove::<ControllingPlayer>();
            }

            // goal keeper is now the controller
            commands.entity(entity).insert(ControllingPlayer);

            goal_keeper.state_machine.change_state(
                &mut commands,
                entity,
                GoalKeeperState::PutBallBackInPlay,
            );

            return;
        }

        // if the ball is close, move out to try and intercept it
        if goal_keeper.goal_keeper.is_ball_within_range_for_intercept(
            params,
            &goal,
            ball_physical.transform,
        ) {
            goal_keeper.state_machine.change_state(
                &mut commands,
                entity,
                GoalKeeperState::InterceptBall,
            );

            return;
        }

        let is_team_controlling = controlling.optional_single().is_some();

        // if the keeper moved too far out, move back towards the goal
        if goal_keeper.goal_keeper.is_too_far_from_goal_mouth(
            params,
            physical.transform,
            &goal,
            ball_physical.transform,
        ) && is_team_controlling
        {
            goal_keeper.state_machine.change_state(
                &mut commands,
                entity,
                GoalKeeperState::ReturnHome,
            );
        }
    }
}

pub fn TendGoal_exit<T>(
    mut commands: Commands,
    goal_keeper: Query<(Entity, GoalKeeperQuery<T>), With<GoalKeeperStateTendGoalExit>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, goal_keeper)) = goal_keeper.optional_single() {
        goal_keeper.agent.interpose_off(&mut commands, entity);
    }
}

pub fn ReturnHome_enter<T>(
    mut commands: Commands,
    goal_keeper: Query<(Entity, GoalKeeperQuery<T>), With<GoalKeeperStateReturnHomeEnter>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, goal_keeper)) = goal_keeper.optional_single() {
        goal_keeper.agent.arrive_on(&mut commands, entity);

        info!("{} enters return home state", goal_keeper.name);
    }
}

pub fn ReturnHome_execute<T>(
    mut commands: Commands,
    pitch: Res<Pitch>,
    mut goal_keeper: Query<
        (Entity, GoalKeeperQueryMut<T>, &Transform),
        With<GoalKeeperStateReturnHomeExecute>,
    >,
    controlling: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut goal_keeper, transform)) = goal_keeper.optional_single_mut() {
        goal_keeper.steering.target = goal_keeper.player.get_home_region(&pitch).position;

        let position = transform.translation.truncate();
        if goal_keeper
            .player
            .get_home_region(&pitch)
            .is_inside(position)
            && controlling.optional_single().is_some()
        {
            goal_keeper.state_machine.change_state(
                &mut commands,
                entity,
                GoalKeeperState::TendGoal,
            );
        }
    }
}

pub fn ReturnHome_exit<T>(
    mut commands: Commands,
    goal_keeper: Query<(Entity, GoalKeeperQuery<T>), With<GoalKeeperStateReturnHomeExit>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, goal_keeper)) = goal_keeper.optional_single() {
        goal_keeper.agent.arrive_off(&mut commands, entity);
    }
}

pub fn InterceptBall_enter<T>(
    mut commands: Commands,
    goal_keeper: Query<(Entity, GoalKeeperQuery<T>), With<GoalKeeperStateInterceptBallEnter>>,
    ball: Query<Entity, With<Ball>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, goal_keeper)) = goal_keeper.optional_single() {
        goal_keeper
            .agent
            .pursuit_on(&mut commands, entity, ball.single());

        info!("{} enters intercept ball state", goal_keeper.name);
    }
}

pub fn InterceptBall_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut goal_keeper: Query<
        (
            Entity,
            GoalKeeperQueryMut<T>,
            PhysicalQuery,
            Option<&ClosestPlayer>,
        ),
        (With<GoalKeeperStateInterceptBallExecute>, Without<Ball>),
    >,
    controlling: Query<ControllingPlayerQuery<T>>,
    closest_opponent: Query<&Transform, (With<ClosestPlayer>, Without<T>)>,
    goal: Query<TeamGoalQuery<T>>,
    mut ball: Query<PhysicalQueryMut, With<Ball>>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    if let Some((entity, mut goal_keeper, physical, closest)) = goal_keeper.optional_single_mut() {
        let mut ball_physical = ball.single_mut();
        let ball_position = ball_physical.transform.translation.truncate();
        let goal = goal.single();

        let is_closest_player = if closest.is_some() {
            if let Some(closest_opponent) = closest_opponent.optional_single() {
                let position = physical.transform.translation.truncate();
                let dist_to_ball = position.distance_squared(ball_position);

                let opponent_position = closest_opponent.translation.truncate();
                let opponent_distance_to_ball = opponent_position.distance_squared(ball_position);
                dist_to_ball < opponent_distance_to_ball
            } else {
                true
            }
        } else {
            false
        };

        // if the keeper moved too far out, move back towards the goal
        // unless they're the closest player to the ball
        if goal_keeper.goal_keeper.is_too_far_from_goal_mouth(
            params,
            physical.transform,
            &goal,
            ball_physical.transform,
        ) && !is_closest_player
        {
            goal_keeper.state_machine.change_state(
                &mut commands,
                entity,
                GoalKeeperState::ReturnHome,
            );

            return;
        }

        // if the ball comes in range, trap it and put it back in play
        if goal_keeper.goal_keeper.is_ball_within_keeper_range(
            params,
            physical.transform,
            ball_physical.transform,
        ) {
            ball_physical.physical.velocity = Vec2::ZERO;

            if let Some(controlling) = controlling.optional_single() {
                commands
                    .entity(controlling.entity)
                    .remove::<ControllingPlayer>();
            }

            // goal keeper is now the controller
            commands.entity(entity).insert(ControllingPlayer);

            goal_keeper.state_machine.change_state(
                &mut commands,
                entity,
                GoalKeeperState::PutBallBackInPlay,
            );
        }
    }
}

pub fn InterceptBall_exit<T>(
    mut commands: Commands,
    goal_keeper: Query<(Entity, GoalKeeperQuery<T>), With<GoalKeeperStateInterceptBallExit>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, goal_keeper)) = goal_keeper.optional_single() {
        goal_keeper.agent.pursuit_off(&mut commands, entity);
    }
}

pub fn PutBallBackInPlay_enter<T>(
    mut commands: Commands,
    mut player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    goal_keeper: Query<(Entity, GoalKeeperQuery<T>), With<GoalKeeperStatePutBallBackInPlayEnter>>,
    field_players: Query<Entity, With<FieldPlayer>>,
    controlling: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, goal_keeper)) = goal_keeper.optional_single() {
        if let Some(controlling) = controlling.optional_single() {
            commands
                .entity(controlling.entity)
                .remove::<ControllingPlayer>();
        }

        // goal keeper is now the controller
        commands.entity(entity).insert(ControllingPlayer);

        // send all the field players home
        SoccerTeam::send_all_field_players_home(
            &mut player_message_dispatcher,
            field_players.iter(),
        );

        info!("{} enters put ball back in play state", goal_keeper.name);
    }
}

pub fn PutBallBackInPlay_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut field_player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    mut goal_keeper: Query<
        (Entity, GoalKeeperQueryMut<T>, PhysicalQueryMut),
        (With<GoalKeeperStatePutBallBackInPlayExecute>, Without<Ball>),
    >,
    team: Query<SoccerTeamQuery<T>>,
    teammates: Query<
        (Entity, FieldPlayerQuery<T>, PhysicalQuery),
        Without<GoalKeeperStatePutBallBackInPlayExecute>,
    >,
    mut ball: Query<(&Ball, PhysicalQueryMut, &BoundingCircle), Without<SoccerPlayer>>,
    opponent_goal: Query<GoalQuery, Without<T>>,
    opponents: Query<(PhysicalQuery, &BoundingCircle), (With<SoccerPlayer>, Without<T>)>,
) where
    T: TeamColorMarker,
{
    let params = params_assets.get(&params_asset.handle).unwrap();

    if let Some((entity, mut goal_keeper, mut physical)) = goal_keeper.optional_single_mut() {
        let team = team.single();
        let opponent_goal = opponent_goal.single();

        let (ball, mut ball_physical, ball_bounds) = ball.single_mut();
        let ball_position = ball_physical.transform.translation.truncate();

        // try to find a safe player to pass to
        let (receiver, ball_target) = team.team.find_pass::<T, _, _, _>(
            params,
            (entity, physical.transform),
            teammates.iter(),
            || opponents.iter(),
            &opponent_goal,
            (
                &ball_physical.physical,
                ball_physical.transform,
                ball_bounds,
            ),
            params.max_passing_force,
            params.goal_keeper_min_pass_distance,
        );
        if let Some(receiver) = receiver {
            let direction = ball_target - ball_position;
            ball.kick(
                &mut ball_physical.physical,
                direction,
                params.max_passing_force,
            );

            commands.entity(entity).remove::<ControllingPlayer>();

            field_player_message_dispatcher
                .dispatch_message(Some(receiver), FieldPlayerMessage::ReceiveBall(ball_target));

            goal_keeper.state_machine.change_state(
                &mut commands,
                entity,
                GoalKeeperState::TendGoal,
            );

            return;
        }

        physical.physical.velocity = Vec2::ZERO;
    }
}
