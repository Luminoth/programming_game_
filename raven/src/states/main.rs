use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_prototype_lyon::prelude::*;

use crate::components::camera::*;
use crate::resources::game::*;
use crate::{CAMERA_SCALE, PIXELS_TO_UNITS};

pub fn setup(mut commands: Commands) {
    debug!("entering main state");

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
    camera.orthographic_projection.scale = CAMERA_SCALE;
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    // map
    commands.insert_resource(Map);

    // nav
    commands.insert_resource(NavGraph);

    // objects
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                extents: Vec2::new(-1.0, 1.0),
                ..Default::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::WHITE, 1.0 * PIXELS_TO_UNITS),
            },
            Transform::default(),
        ))
        .insert(Name::new("Test Object"));
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    debug!("leaving main state");

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
