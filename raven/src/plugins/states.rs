use bevy::app::PluginGroupBuilder;
use bevy::core::FixedTimestep;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

use crate::game::PHYSICS_STEP;
use crate::states;
use crate::states::*;
use crate::systems;
use crate::systems::Systems;

pub struct StatesPlugins;

impl PluginGroup for StatesPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(IntroStatePlugin).add(MainStatePlugin);
    }
}

struct IntroStatePlugin;

impl Plugin for IntroStatePlugin {
    fn build(&self, app: &mut App) {
        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Intro).with_system(states::intro::setup))
            .add_system_set(
                SystemSet::on_update(GameState::Intro).with_system(states::intro::button_handler),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Intro).with_system(states::intro::teardown),
            );
    }
}

struct MainStatePlugin;

impl Plugin for MainStatePlugin {
    fn build(&self, app: &mut App) {
        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(states::main::setup))
            // physics (fixed timestep)
            .add_system_set(
                // https://github.com/bevyengine/bevy/issues/1839
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64).chain(
                        |In(input): In<ShouldRun>, state: Res<State<GameState>>| {
                            if state.current() == &GameState::Main {
                                input
                            } else {
                                ShouldRun::No
                            }
                        },
                    ))
                    /*SystemSet::on_update(GameState::Main)
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))*/
                    // physics
                    .with_system(systems::physics::update.label(Systems::Physics))
                    // bounds checking
                    .with_system(
                        systems::projectile::check_bounds
                            .label(Systems::BoundsCheck)
                            .after(Systems::Physics),
                    )
                    .with_system(
                        systems::projectile::check_collision
                            .label(Systems::Collision)
                            .before(Systems::Physics),
                    ),
            )
            // per-frame systems
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    // input
                    .with_system(systems::input::select_bot.label(Systems::Input))
                    .with_system(systems::input::deselect_bot.label(Systems::Input))
                    .with_system(systems::input::select_weapon.label(Systems::Input))
                    .with_system(systems::input::fire_weapon.label(Systems::Input))
                    .with_system(systems::input::fill_inventory.label(Systems::Input))
                    .with_system(systems::input::damage_bot.label(Systems::Input))
                    .with_system(systems::input::kill_bot.label(Systems::Input))
                    // everything else
                    .with_system(systems::weapons::update.label(Systems::Weapons)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Main).with_system(states::main::teardown),
            );
    }
}
