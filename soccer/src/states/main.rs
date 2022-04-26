use bevy::prelude::*;

use crate::bundles::ball::*;
use crate::bundles::goal::*;
use crate::bundles::pitch::*;
use crate::bundles::team::*;
use crate::components::camera::*;
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
    GoalBundle::spawn(&mut commands, &params, Team::Red, &pitch);
    GoalBundle::spawn(&mut commands, &params, Team::Blue, &pitch);

    // ball
    BallBundle::spawn(&mut commands, Vec2::ZERO);

    // teams
    SoccerTeamBundle::spawn(&mut commands, &params, Team::Red, &pitch);
    SoccerTeamBundle::spawn(&mut commands, &params, Team::Blue, &pitch);

    commands.insert_resource(pitch);

    //info!("prepare for kick off!");

    let /*mut*/ player_message_dispatcher = FieldPlayerMessageDispatcher::default();

    /*player_message_dispatcher.dispatch_message(red, FieldPlayerMessage::GoHome);
    player_message_dispatcher.dispatch_message(blue, FieldPlayerMessage::GoHome);*/

    let /*mut*/ goal_keeper_message_dispatcher = GoalKeeperMessageDispatcher::default();

    /*goal_keeper_message_dispatcher.dispatch_message(red, GoalKeeperMessage::GoHome);
    goal_keeper_message_dispatcher.dispatch_message(blue, GoalKeeperMessage::GoHome);*/

    // messaging
    commands.insert_resource(player_message_dispatcher);
    commands.insert_resource(goal_keeper_message_dispatcher);
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    debug!("leaving main state");

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<GameState>();
    commands.remove_resource::<ClearColor>();
}
