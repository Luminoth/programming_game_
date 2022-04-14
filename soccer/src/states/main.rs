use bevy::prelude::*;

use crate::bundles::ball::*;
use crate::bundles::goal::*;
use crate::bundles::pitch::*;
use crate::bundles::team::*;
use crate::components::camera::*;
use crate::game::team::Team;
use crate::resources::{GameState, SimulationParams};

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

    // pitch
    PitchBundle::spawn(&mut commands, params.pitch_extents);

    // goals
    GoalBundle::spawn(&mut commands, &params, Team::Red);
    GoalBundle::spawn(&mut commands, &params, Team::Blue);

    // ball
    BallBundle::spawn(&mut commands, Vec2::ZERO);

    // teams
    SoccerTeamBundle::spawn(&mut commands, &params, Team::Red);
    SoccerTeamBundle::spawn(&mut commands, &params, Team::Blue);
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    debug!("leaving main state");

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<GameState>();
    commands.remove_resource::<ClearColor>();
}
