use bevy::prelude::*;

use crate::bundles::vehicle::*;
use crate::components::camera::*;
use crate::components::steering::*;

pub fn setup(mut commands: Commands) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    VehicleBundle::spawn(
        &mut commands,
        SteeringBehavior::Seek(Vec2::new(-200.0, 200.0)),
        1.0,
        100.0,
        100.0,
        10.0,
        "seek",
        Color::RED,
    );

    VehicleBundle::spawn(
        &mut commands,
        SteeringBehavior::Flee(Vec2::new(1.0, 1.0)),
        1.0,
        100.0,
        100.0,
        10.0,
        "flee",
        Color::GREEN,
    );

    VehicleBundle::spawn(
        &mut commands,
        SteeringBehavior::Arrive(Vec2::new(200.0, -200.0), Deceleration::Slow),
        1.0,
        100.0,
        100.0,
        10.0,
        "arrive",
        Color::BLUE,
    );
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
