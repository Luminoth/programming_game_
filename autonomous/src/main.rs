#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod bundles;
mod components;
mod events;
mod plugins;
mod resources;
mod states;
mod systems;
mod util;

use bevy::core::FixedTimestep;
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::WorldInspectorParams;
use bevy_prototype_lyon::prelude::*;

use crate::components::physics::PHYSICS_STEP;
use crate::plugins::debug::*;
use crate::resources::ui::*;
use crate::resources::*;
use crate::states::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    // assets
    let fonts = Fonts {
        normal: asset_server.load("fonts/FiraSans-Bold.ttf"),
    };
    commands.insert_resource(fonts);

    commands.insert_resource(SimulationParams {
        window_border: 10.0,
        num_obstacles: 7,
        min_obstacle_radius: 10.0,
        max_obstacle_radius: 30.0,
        min_gap_between_obstacles: 20.0,
        min_detection_box_length: 40.0,
    });
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
    .register_inspectable::<components::physics::Physical>()
    .register_inspectable::<components::steering::Seek>()
    .register_inspectable::<components::steering::SeekTarget>()
    .register_inspectable::<components::steering::Flee>()
    .register_inspectable::<components::steering::FleeTarget>()
    .register_inspectable::<components::steering::Arrive>()
    .register_inspectable::<components::steering::ArriveTarget>()
    .register_inspectable::<components::steering::Pursuit>()
    .register_inspectable::<components::steering::PursuitTarget>()
    .register_inspectable::<components::steering::Evade>()
    .register_inspectable::<components::steering::EvadeTarget>()
    .register_inspectable::<components::steering::Wander>();

    // plugins
    app.add_plugin(DebugPlugin);

    // initial game state
    app.add_state(GameState::Intro);

    // main setup
    app.add_startup_system(setup);

    // intro state
    app.add_system_set(SystemSet::on_enter(GameState::Intro).with_system(states::intro::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Intro).with_system(states::intro::button_handler),
        )
        .add_system_set(SystemSet::on_exit(GameState::Intro).with_system(states::intro::teardown));

    // game state
    app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
        // physics
        .add_system_set(
            SystemSet::on_update(GameState::Main)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                .with_system(systems::steering::update_seek.label("steering"))
                .with_system(systems::steering::update_flee.label("steering"))
                .with_system(systems::steering::update_arrive.label("steering"))
                .with_system(
                    systems::steering::update_pursuit
                        .label("steering")
                        .label("pursuit"),
                )
                .with_system(
                    systems::steering::update_evade
                        .label("steering")
                        .after("pursuit"),
                )
                .with_system(systems::steering::update_wander.label("steering"))
                .with_system(
                    systems::steering::update_obstacle_avoidance
                        .label("avoidance")
                        .after("steering"),
                )
                .with_system(systems::physics::update.label("physics").after("avoidance"))
                .with_system(systems::wrap.after("physics"))
                .with_system(systems::facing.after("avoidance").before("physics")),
        )
        // TODO: non-physics systems here
        //.add_system_set(SystemSet::on_update(GameState::Main).with_system())
        .add_system_set(SystemSet::on_exit(GameState::Main).with_system(states::main::teardown));

    app.run();
}
