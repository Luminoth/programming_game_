use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::bundles::bot::BotBundle;
use crate::components::camera::*;
use crate::resources::game::*;
use crate::ORTHO_SIZE;

pub fn setup(mut commands: Commands) {
    debug!("entering main state");

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
    camera.orthographic_projection.scale = ORTHO_SIZE;
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    // map
    commands.insert_resource(Map);

    // nav
    commands.insert_resource(NavGraph);

    // spawn bots
    // TODO: this should be done using spawnpoints
    BotBundle::spawn_at_position(&mut commands, "Test", Color::WHITE, Vec2::new(0.0, 10.0));
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    debug!("leaving main state");

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<NavGraph>();
    commands.remove_resource::<Map>();
    commands.remove_resource::<ClearColor>();
}
