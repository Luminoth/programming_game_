use bevy::prelude::*;

use crate::bundles::ball::*;
use crate::bundles::goal::*;
use crate::bundles::pitch::*;
use crate::bundles::team::*;
use crate::components::camera::*;
use crate::game::Team;

pub fn setup(mut commands: Commands) {
    debug!("entering main state");

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    // pitch
    PitchBundle::spawn(&mut commands, Vec2::ZERO);

    // goals
    GoalBundle::spawn(&mut commands, Vec2::new(-250.0, 0.0), Team::Red);
    GoalBundle::spawn(&mut commands, Vec2::new(250.0, 0.0), Team::Blue);

    // ball
    BallBundle::spawn(&mut commands, Vec2::ZERO);

    // teams
    spawn_team(
        &mut commands,
        Vec2::new(-100.0, 0.0),
        Vec2::new(-200.0, 0.0),
        Team::Red,
    );
    spawn_team(
        &mut commands,
        Vec2::new(100.0, 0.0),
        Vec2::new(200.0, 0.0),
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
