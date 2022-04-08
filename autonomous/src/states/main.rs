use bevy::prelude::*;

use crate::bundles::vehicle::*;
use crate::components::camera::*;
use crate::components::steering;

pub fn setup(mut commands: Commands) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    let entity = VehicleBundle::spawn(
        &mut commands,
        steering::Seek::default(),
        Vec2::ZERO,
        1.0,
        100.0,
        100.0,
        10.0,
        "seek",
        Color::RED,
    );

    commands.entity(entity).insert(steering::SeekTarget {
        position: Vec2::new(-200.0, 200.0),
    });

    let entity = VehicleBundle::spawn(
        &mut commands,
        steering::Flee::default(),
        Vec2::ZERO,
        1.0,
        100.0,
        100.0,
        10.0,
        "flee",
        Color::GREEN,
    );

    commands.entity(entity).insert(steering::FleeTarget {
        //position: Vec2::ZERO,
        position: Vec2::new(1.0, 1.0),
    });

    let entity = VehicleBundle::spawn(
        &mut commands,
        steering::Arrive {
            deceleration: steering::Deceleration::Slow,
        },
        Vec2::ZERO,
        1.0,
        100.0,
        100.0,
        10.0,
        "arrive",
        Color::BLUE,
    );

    commands.entity(entity).insert(steering::ArriveTarget {
        position: Vec2::new(200.0, -200.0),
    });

    let evade_entity = VehicleBundle::spawn(
        &mut commands,
        steering::Evade::default(),
        //Vec2::ZERO,
        Vec2::new(10.0, 10.0),
        1.0,
        100.0,
        100.0,
        10.0,
        "evade",
        Color::PINK,
    );

    let pursuit_entity = VehicleBundle::spawn(
        &mut commands,
        steering::Pursuit::default(),
        //Vec2::ZERO,
        Vec2::new(-10.0, -10.0),
        1.0,
        75.0,
        75.0,
        10.0,
        "pursuit",
        Color::PURPLE,
    );

    commands.entity(evade_entity).insert(steering::EvadeTarget {
        entity: pursuit_entity,
    });

    commands
        .entity(pursuit_entity)
        .insert(steering::PursuitTarget {
            entity: evade_entity,
        });

    VehicleBundle::spawn(
        &mut commands,
        steering::Wander::new(100.0, 100.0, 50.0),
        Vec2::ZERO,
        1.0,
        100.0,
        100.0,
        10.0,
        "wander",
        Color::YELLOW,
    );
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
