#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod bundles;
mod components;
mod events;
mod game;
mod plugins;
mod resources;
mod states;
mod systems;
mod util;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::WorldInspectorParams;
use bevy_prototype_lyon::prelude::*;

use plugins::debug::DebugPlugin;
use plugins::states::StatesPlugins;
use resources::SimulationParams;
use states::GameState;

pub const BALL_SORT: f32 = 2.0;
pub const GOAL_SORT: f32 = 2.0;
pub const PITCH_SORT: f32 = 0.0;
pub const BORDER_SORT: f32 = 3.0;
pub const PLAYER_SORT: f32 = 2.0;
pub const AGENT_UPDATE_STEP: f64 = 0.8;
pub const DEBUG_SORT: f32 = 100.0;
pub const DEBUG_RADIUS: f32 = 5.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    commands.insert_resource(SimulationParams {
        pitch_extents: Vec2::new(900.0, 450.0),
        goal_extents: Vec2::new(40.0, 90.0),

        num_regions_horizontal: 6,
        num_regions_vertical: 3,

        num_support_spots_horizontal: 13,
        num_support_spots_vertical: 6,

        max_passing_force: 3.0,

        num_attempts_to_find_valid_strike: 5,
        pass_safe_score: 2.0,
        can_score_score: 1.0,
        distance_from_controller_player_score: 2.0,

        // NOTE: this is negative in the example source
        // so anywhere it's used, it needs to be negated
        friction: 0.015,

        debug_vis: false,
    });
}

#[bevy_main]
fn main() {
    let mut app = App::new();

    // basic bevy
    app.insert_resource(WindowDescriptor {
        title: "Soccer".to_owned(),
        width: 1024.0,
        height: 768.0,
        present_mode: PresentMode::Immediate,
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
    .register_inspectable::<game::team::Team>()
    .register_inspectable::<components::physics::Physical>()
    .register_inspectable::<components::actor::Actor>()
    .register_inspectable::<components::obstacle::Obstacle>()
    .register_inspectable::<components::obstacle::Wall>()
    .register_inspectable::<components::steering::Steering>()
    .register_inspectable::<components::steering::ObstacleAvoidance>()
    .register_inspectable::<components::ball::Ball>()
    .register_inspectable::<components::goal::Goal>()
    .register_inspectable::<components::pitch::PitchBorder>()
    .register_inspectable::<components::team::SoccerTeam>()
    .register_inspectable::<components::team::SupportSpot>()
    .register_inspectable::<components::team::SupportSpotCalculator>()
    .register_inspectable::<components::team::FieldPlayer>()
    .register_inspectable::<components::team::GoalKeeper>()
    .register_inspectable::<components::team::ReceivingPlayer>()
    .register_inspectable::<components::team::ClosestPlayer>()
    .register_inspectable::<components::team::ControllingPlayer>()
    .register_inspectable::<components::team::SupportingPlayer>();

    // plugins
    app.add_plugin(DebugPlugin).add_plugins(StatesPlugins);

    // initial game state
    app.add_state(GameState::Main);

    // main setup
    app.add_startup_system(setup);

    app.run();
}
