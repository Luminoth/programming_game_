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

use bevy::diagnostic::*;
use bevy::prelude::*;
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    commands.insert_resource(SimulationParams {
        pitch_extents: Vec2::new(900.0, 450.0),
        goal_extents: Vec2::new(40.0, 90.0),

        // NOTE: this is negative in the example source
        // so anywhere it's used, it needs to be negated
        friction: 0.015,
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

    // inspectorActor
    app.insert_resource(WorldInspectorParams {
        enabled: false,
        despawnable_entities: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new())
    // inspectable types
    .register_inspectable::<game::team::Team>()
    .register_inspectable::<game::team::SoccerTeamState>()
    .register_inspectable::<game::team::FieldPlayerState>()
    .register_inspectable::<game::team::GoalieState>()
    .register_inspectable::<components::physics::Physical>()
    .register_inspectable::<components::actor::Actor>()
    .register_inspectable::<components::obstacle::Obstacle>()
    .register_inspectable::<components::obstacle::Wall>()
    .register_inspectable::<components::steering::Steering>()
    .register_inspectable::<components::steering::ObstacleAvoidance>()
    .register_inspectable::<components::ball::Ball>()
    .register_inspectable::<components::ball::BallOwner>()
    .register_inspectable::<components::goal::Goal>()
    .register_inspectable::<components::pitch::Pitch>()
    .register_inspectable::<components::pitch::PitchBorder>()
    .register_inspectable::<components::team::SoccerTeam>()
    .register_inspectable::<components::team::SoccerTeamStateMachine>()
    .register_inspectable::<components::team::FieldPlayer>()
    .register_inspectable::<components::team::FieldPlayerStateMachine>()
    .register_inspectable::<components::team::Goalie>()
    .register_inspectable::<components::team::GoalieStateMachine>();

    // plugins
    app.add_plugin(DebugPlugin).add_plugins(StatesPlugins);

    // initial game state
    app.add_state(GameState::Main);

    // main setup
    app.add_startup_system(setup);

    app.run();
}
