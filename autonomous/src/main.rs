mod bundles;
mod components;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::vehicle::VehicleBundle;
use crate::components::steering::*;

fn setup(mut commands: Commands) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Name::new("Camera"));

    VehicleBundle::spawn(&mut commands, "test");
}

#[bevy_main]
fn main() {
    let mut app = App::new();

    // basic bevy
    app.insert_resource(WindowDescriptor {
        title: "Autonomous Agent".to_owned(),
        width: 1024.0,
        height: 768.0,
        vsync: false,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::INFO,
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins);

    // prototype lyon
    app.add_plugin(ShapePlugin);

    // main setup
    app.add_startup_system(setup);

    // physics
    app.add_system(systems::steering::update_steering::<SteeringTest>.label("steering"))
        .add_system(systems::physics::update.label("physics").after("steering"));

    app.run();
}
