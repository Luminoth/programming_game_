use bevy::prelude::*;

use crate::bundles::ball::*;
use crate::bundles::goal::*;
use crate::bundles::pitch::*;
use crate::bundles::team::*;
use crate::components::camera::*;
use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::*;

pub fn setup(
    mut commands: Commands,
    params_asset: Res<SimulationParamsAsset>,
    mut params_assets: ResMut<Assets<SimulationParams>>,
) {
    debug!("entering main state");

    // init the simulation params
    let params = params_assets.get_mut(&params_asset.handle).unwrap();

    let force_tweaker = 200.0;
    let speed_tweaker = 125.0;

    params.player_max_force *= force_tweaker;
    params.player_max_speed_without_ball *= speed_tweaker;
    params.player_max_speed_with_ball *= speed_tweaker;

    params.ball_max_force *= force_tweaker;
    params.ball_max_speed *= speed_tweaker;

    params.ball_within_receiving_range_squared =
        params.ball_within_receiving_range * params.ball_within_receiving_range;
    params.player_in_target_range_squared =
        params.player_in_target_range * params.player_in_target_range;
    params.player_kicking_distance_squared =
        params.player_kicking_distance * params.player_kicking_distance;
    params.player_comfort_zone_squared = params.player_comfort_zone * params.player_comfort_zone;
    params.keeper_in_ball_range_squared = params.keeper_in_ball_range * params.keeper_in_ball_range;

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    // game state
    commands.insert_resource(GameState::default());

    let pitch = Pitch::new(&params);

    // pitch
    PitchBundle::spawn(&mut commands, &params, &pitch);

    // goals
    GoalBundle::spawn(&mut commands, &params, RedTeam, &pitch);
    GoalBundle::spawn(&mut commands, &params, BlueTeam, &pitch);

    // ball
    BallBundle::spawn(&mut commands, &params, Vec2::ZERO);

    // teams
    SoccerTeamBundle::<RedTeam>::spawn(&mut commands, &params, &pitch);
    SoccerTeamBundle::<BlueTeam>::spawn(&mut commands, &params, &pitch);

    commands.insert_resource(pitch);

    // messaging
    commands.insert_resource(FieldPlayerMessageDispatcher::default());
    commands.insert_resource(GoalKeeperMessageDispatcher::default());
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    debug!("leaving main state");

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<GameState>();
    commands.remove_resource::<ClearColor>();
}
