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
use resources::ui::*;
use states::GameState;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;

const ORTHO_SIZE: f32 = 50.0; // Unity's Camera.orthographicSize (half the size of the vertical viewing volume)
pub const CAMERA_SCALE: f32 = ORTHO_SIZE / (WINDOW_WIDTH / WINDOW_HEIGHT); // Bevy's OrthographicProject scale value

// units to pixels conversion
pub const UNITS_TO_PIXELS: f32 = ORTHO_SIZE * ORTHO_SIZE;
pub const PIXELS_TO_UNITS: f32 = 1.0 / UNITS_TO_PIXELS;

// sprite sorting
pub const BOT_SORT: f32 = 2.0;
pub const PROJECTILE_SORT: f32 = 2.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    commands.insert_resource(Fonts {
        normal: asset_server.load("fonts/FiraSans-Bold.ttf"),
    });
}

#[bevy_main]
fn main() {
    let mut app = App::new();

    // basic bevy
    app.insert_resource(WindowDescriptor {
        title: "Raven".to_owned(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
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
    .register_inspectable::<components::actor::Actor>()
    .register_inspectable::<components::agent::Agent>()
    .register_inspectable::<components::physics::Physical>()
    .register_inspectable::<components::projectile::Bolt>()
    .register_inspectable::<components::projectile::Pellet>()
    .register_inspectable::<components::projectile::Rocket>()
    .register_inspectable::<components::projectile::Slug>()
    .register_inspectable::<components::weapon::Blaster>()
    .register_inspectable::<components::weapon::Shotgun>()
    .register_inspectable::<components::weapon::RocketLauncher>()
    .register_inspectable::<components::weapon::Railgun>()
    .register_inspectable::<components::world::SpawnPoint>()
    .register_inspectable::<components::world::Wall>();

    // plugins
    app.add_plugin(DebugPlugin).add_plugins(StatesPlugins);

    // initial game state
    app.add_state(GameState::Intro);

    // main setup
    app.add_startup_system(setup);

    app.run();
}
