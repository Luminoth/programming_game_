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

pub fn setup(mut commands: Commands, params: Res<SimulationParams>) {
    debug!("entering main state");

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
