mod bundles;
mod components;
mod events;
mod plugins;
mod resources;
mod systems;

use bevy::core::FixedTimestep;
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::WorldInspectorParams;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::vehicle::*;
use crate::components::physics::PHYSICS_STEP;
use crate::components::steering::*;
use crate::plugins::debug::*;

fn setup(mut commands: Commands) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Name::new("Camera"));

    VehicleBundle::spawn(
        &mut commands,
        SteeringBehavior::Test,
        1.0,
        100.0,
        100.0,
        10.0,
        "test",
    );
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
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin);

    // prototype lyon
    app.add_plugin(ShapePlugin);

    // egui
    app.insert_resource(EguiSettings { scale_factor: 0.75 })
        .add_plugin(EguiPlugin);

    // inspector
    app.insert_resource(WorldInspectorParams {
        enabled: false,
        despawnable_entities: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new())
    // inspectable types
    .register_inspectable::<components::physics::Physical>();

    // plugins
    app.add_plugin(DebugPlugin);

    // main setup
    app.add_startup_system(setup);

    // physics
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
            .with_system(systems::steering::update_steering.label("steering"))
            .with_system(systems::physics::update.label("physics").after("steering")),
    )
    .add_system(systems::wrap.after("physics"));

    app.run();
}
