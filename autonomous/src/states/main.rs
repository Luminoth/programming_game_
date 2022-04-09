use bevy::prelude::*;
use rand::Rng;

use crate::bundles::obstacle::*;
use crate::bundles::vehicle::*;
use crate::components::camera::*;
use crate::components::steering;
use crate::resources::*;
use crate::util;

pub fn setup(mut commands: Commands, params: Res<SimulationParams>, window: Res<WindowDescriptor>) {
    let mut rng = rand::thread_rng();

    let hw = window.width * 0.5;
    let hh = window.height * 0.5;

    let min_x = -hw + VEHICLE_RADIUS + params.window_border;
    let max_x = hw - VEHICLE_RADIUS - params.window_border;
    let min_y = -hh + VEHICLE_RADIUS + params.window_border;
    let max_y = hh - VEHICLE_RADIUS - params.window_border;

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    // vehicles
    let entity = VehicleBundle::spawn(
        &mut commands,
        steering::Seek::default(),
        Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y)),
        1.0,
        100.0,
        100.0,
        10.0,
        "seek",
        Color::RED,
    );

    commands.entity(entity).insert(steering::SeekTarget {
        position: Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y)),
    });

    // TODO: fleeing seems like it may not be working quite right
    // based on how it has to be setup to flee from something right next to it
    // and once it starts fleeing, it never stops
    let flee_position = Vec2::new(
        rng.gen_range(
            -hw + VEHICLE_RADIUS + params.window_border..hw - VEHICLE_RADIUS - params.window_border,
        ),
        rng.gen_range(
            -hh + VEHICLE_RADIUS + params.window_border..hh - VEHICLE_RADIUS - params.window_border,
        ),
    );

    let entity = VehicleBundle::spawn(
        &mut commands,
        steering::Flee::default(),
        flee_position,
        1.0,
        100.0,
        100.0,
        10.0,
        "flee",
        Color::GREEN,
    );

    commands.entity(entity).insert(steering::FleeTarget {
        position: flee_position + Vec2::new(1.0, 1.0),
    });

    let entity = VehicleBundle::spawn(
        &mut commands,
        steering::Arrive {
            deceleration: steering::Deceleration::Slow,
        },
        Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y)),
        1.0,
        100.0,
        100.0,
        10.0,
        "arrive",
        Color::BLUE,
    );

    commands.entity(entity).insert(steering::ArriveTarget {
        position: Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y)),
    });

    let evade_entity = VehicleBundle::spawn(
        &mut commands,
        steering::Evade::default(),
        Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y)),
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
        Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y)),
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
        Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y)),
        1.0,
        100.0,
        100.0,
        10.0,
        "wander",
        Color::YELLOW,
    );

    // build a set of non-overlapping obstacles
    let max_tries = 2000;
    let mut obstacles = Vec::with_capacity(params.num_obstacles);
    for _ in 0..params.num_obstacles {
        let mut num_tries = 0;
        let mut overlapped = true;
        while overlapped {
            num_tries += 1;
            if num_tries > max_tries {
                break;
            }

            let radius = rng.gen_range(params.min_obstacle_radius..=params.max_obstacle_radius);
            let min_x = -hw + radius + params.window_border;
            let max_x = hw - radius - params.window_border;
            let min_y = -hh + radius + params.window_border;
            let max_y = hh - radius - params.window_border;
            let position = Vec2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y));

            if !util::overlapped(
                position,
                radius,
                &obstacles,
                params.min_gap_between_obstacles,
            ) {
                obstacles.push((position, radius));
                overlapped = false;
            }
        }

        if num_tries > max_tries {
            break;
        }
    }

    // spawn the obstacles
    for (position, radius) in obstacles {
        ObstacleBundle::spawn(&mut commands, position, radius);
    }
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
