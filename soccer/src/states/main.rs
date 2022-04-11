use bevy::prelude::*;

use crate::bundles::ball::*;
use crate::bundles::goal::*;
use crate::bundles::pitch::*;
use crate::bundles::team::*;
use crate::components::camera::*;
use crate::game::{Team, GOALIE_PAD, PLAYER_RADIUS, TEAM_SPREAD};
use crate::resources::SimulationParams;

pub fn setup(mut commands: Commands, params: Res<SimulationParams>) {
    debug!("entering main state");

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    // pitch
    PitchBundle::spawn(&mut commands, params.pitch_extents);

    // goals
    let hw = params.pitch_extents.x * 0.5 - params.goal_extents.x * 0.5;
    GoalBundle::spawn(
        &mut commands,
        Vec2::new(-hw, 0.0),
        params.goal_extents,
        Team::Red,
    );
    GoalBundle::spawn(
        &mut commands,
        Vec2::new(hw, 0.0),
        params.goal_extents,
        Team::Blue,
    );

    // ball
    BallBundle::spawn(&mut commands, Vec2::ZERO);

    // teams
    let hw = params.pitch_extents.x * 0.5 - params.goal_extents.x - PLAYER_RADIUS - GOALIE_PAD;
    spawn_team(
        &mut commands,
        Vec2::new(-TEAM_SPREAD, 0.0),
        Vec2::new(-hw, 0.0),
        Team::Red,
    );
    spawn_team(
        &mut commands,
        Vec2::new(TEAM_SPREAD, 0.0),
        Vec2::new(hw, 0.0),
        Team::Blue,
    );
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    debug!("leaving main state");

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
