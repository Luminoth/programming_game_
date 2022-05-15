#![allow(non_snake_case)]

pub mod field_player;
pub mod goal_keeper;

use bevy::prelude::*;

use crate::components::actor::*;
use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::*;
use crate::util::*;

pub fn update<T>(
    mut commands: Commands,
    teams: Query<SoccerTeamQuery<T>>,
    players: Query<(Entity, &Transform), (With<SoccerPlayer>, With<T>)>,
    closest: Query<ClosestPlayerQuery<T>>,
    ball: Query<PhysicalQuery, With<Ball>>,
) where
    T: TeamColorMarker,
{
    for team in teams.iter() {
        team.team.calculate_closest_player_to_ball::<T, _>(
            &mut commands,
            ball.single().transform.translation.truncate(),
            players.iter(),
            closest.optional_single().map(|x| x.entity),
        );
    }
}

pub fn PrepareForKickOff_enter<T>(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut player_message_dispatcher: ResMut<FieldPlayerMessageDispatcher>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStatePrepareForKickOffEnter>>,
    receiving: Query<ReceivingPlayerQuery<T>>,
    closest: Query<ClosestPlayerQuery<T>>,
    controlling: Query<ControllingPlayerQuery<T>>,
    supporting: Query<SupportingPlayerQuery<T>>,
    field_players: Query<Entity, (With<FieldPlayer>, With<T>)>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        info!("{:?} team preparing for kick off", team.color.team_color());

        match team.color.team_color() {
            TeamColor::Red => game_state.red_team_ready = false,
            TeamColor::Blue => game_state.blue_team_ready = false,
        }

        // reset player positions

        if let Some(receiving) = receiving.optional_single() {
            commands
                .entity(receiving.entity)
                .remove::<ReceivingPlayer>();
        }

        if let Some(closest) = closest.optional_single() {
            commands.entity(closest.entity).remove::<ClosestPlayer>();
        }

        if let Some(controlling) = controlling.optional_single() {
            commands
                .entity(controlling.entity)
                .remove::<ControllingPlayer>();
        }

        if let Some(supporting) = supporting.optional_single() {
            commands
                .entity(supporting.entity)
                .remove::<SupportingPlayer>();
        }

        // send field players home
        SoccerTeam::send_all_field_players_home(
            &mut player_message_dispatcher,
            field_players.iter(),
        );
    }
}

pub fn PrepareForKickOff_execute<T>(
    mut commands: Commands,
    pitch: Res<Pitch>,
    mut teams: Query<
        (Entity, SoccerTeamQueryMut<T>),
        With<SoccerTeamStatePrepareForKickOffExecute>,
    >,
    players: Query<(SoccerPlayerQuery<T>, &Transform)>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut team)) = teams.optional_single_mut() {
        info!("waiting for teams ready ...");

        for (player, transform) in players.iter() {
            if !player.player.is_in_home_region(transform, &pitch) {
                return;
            }
        }

        team.state_machine
            .change_state(&mut commands, entity, SoccerTeamState::Defending);
    }
}

pub fn PrepareForKickOff_exit<T>(
    mut game_state: ResMut<GameState>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStatePrepareForKickOffExit>>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        match team.color.team_color() {
            TeamColor::Red => game_state.red_team_ready = true,
            TeamColor::Blue => game_state.blue_team_ready = true,
        }
    }
}

pub fn Defending_enter<T>(
    pitch: Res<Pitch>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStateDefendingEnter>>,
    mut field_players: Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
    mut goal_keeper: Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        info!("{:?} team defending", team.color.team_color());

        let home_regions = match team.color.team_color() {
            TeamColor::Red => RED_TEAM_DEFENDING_HOME_REGIONS,
            TeamColor::Blue => BLUE_TEAM_DEFENDING_HOME_REGIONS,
        };

        let mut goal_keeper = goal_keeper.single_mut();

        team.team
            .reset_player_home_regions(&mut field_players, &mut goal_keeper, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut field_players);
    }
}

pub fn Defending_execute<T>(
    mut commands: Commands,
    mut teams: Query<(Entity, SoccerTeamQueryMut<T>), With<SoccerTeamStateDefendingExecute>>,
    controller: Query<ControllingPlayerQuery<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut team)) = teams.optional_single_mut() {
        if controller.optional_single().is_some() {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Attacking);
        }
    }
}

pub fn Attacking_enter<T>(
    pitch: Res<Pitch>,
    teams: Query<SoccerTeamQuery<T>, With<SoccerTeamStateAttackingEnter>>,
    mut field_players: Query<FieldPlayerQueryMut<T>, Without<GoalKeeper>>,
    mut goal_keeper: Query<GoalKeeperQueryMut<T>, Without<FieldPlayer>>,
) where
    T: TeamColorMarker,
{
    if let Some(team) = teams.optional_single() {
        info!("{:?} team attacking", team.color.team_color());

        let home_regions = match team.color.team_color() {
            TeamColor::Red => RED_TEAM_ATTACKING_HOME_REGIONS,
            TeamColor::Blue => BLUE_TEAM_ATTACKING_HOME_REGIONS,
        };

        let mut goal_keeper = goal_keeper.single_mut();

        team.team
            .reset_player_home_regions(&mut field_players, &mut goal_keeper, home_regions);

        team.team
            .update_targets_of_waiting_players(&pitch, &mut field_players);
    }
}

pub fn Attacking_execute<T>(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut teams: Query<
        (Entity, SoccerTeamQueryMut<T>, &mut SupportSpotCalculator),
        With<SoccerTeamStateAttackingExecute>,
    >,
    opponents: Query<(&Actor, PhysicalQuery), (With<SoccerPlayer>, Without<T>)>,
    controller: Query<&Transform, (With<T>, With<ControllingPlayer>)>,
    support: Query<SupportingPlayerQuery<T>>,
    ball: Query<(&Actor, &Physical), With<Ball>>,
    opponent_goal: Query<(&Goal, &Transform), Without<T>>,
) where
    T: TeamColorMarker,
{
    if let Some((entity, mut team, mut support_calculator)) = teams.optional_single_mut() {
        let params = params_assets.get(&params_asset.handle).unwrap();

        if let Some(controller_transform) = controller.optional_single() {
            team.team.determine_best_supporting_position(
                params,
                team.color,
                &mut support_calculator,
                || opponents.iter(),
                controller_transform,
                support.optional_single().is_some(),
                ball.single(),
                opponent_goal.single(),
            );
        } else {
            team.state_machine
                .change_state(&mut commands, entity, SoccerTeamState::Defending);
        }
    }
}

pub fn Attacking_exit<T>(
    mut commands: Commands,
    support: Query<Entity, (With<SupportingPlayer>, With<T>)>,
) where
    T: TeamColorMarker,
{
    if let Some(support) = support.optional_single() {
        commands.entity(support).remove::<SupportingPlayer>();
    }
}
