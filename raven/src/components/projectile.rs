use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::BOLT_RADIUS;

// TODO: pull projectile parameters from a config

pub trait Projectile: Default + Component {
    fn name() -> &'static str;

    fn mass() -> f32;

    fn spawn_model(commands: &mut EntityCommands);

    fn damage(&self) -> usize;
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Bolt;

impl Projectile for Bolt {
    fn name() -> &'static str {
        "Bolt"
    }

    fn mass() -> f32 {
        1.0
    }

    fn spawn_model(commands: &mut EntityCommands) {
        commands.insert_bundle(GeometryBuilder::build_as(
            // TODO: this is the wrong shape for a bolt
            // (a green bolt of electricity)
            &shapes::Circle {
                radius: BOLT_RADIUS,
                ..Default::default()
            },
            DrawMode::Fill(FillMode {
                color: Color::LIME_GREEN,
                options: FillOptions::default(),
            }),
            Transform::default(),
        ));
    }

    fn damage(&self) -> usize {
        1
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Pellet;

impl Projectile for Pellet {
    fn name() -> &'static str {
        "Pellet"
    }

    fn mass() -> f32 {
        1.0
    }

    fn spawn_model(commands: &mut EntityCommands) {
        todo!();
    }

    fn damage(&self) -> usize {
        1
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Rocket;

impl Projectile for Rocket {
    fn name() -> &'static str {
        "Rocket"
    }

    fn mass() -> f32 {
        1.0
    }

    fn spawn_model(commands: &mut EntityCommands) {
        todo!();
    }

    fn damage(&self) -> usize {
        10
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Slug;

impl Projectile for Slug {
    fn name() -> &'static str {
        "Slug"
    }

    fn mass() -> f32 {
        1.0
    }

    fn spawn_model(commands: &mut EntityCommands) {
        todo!();
    }

    fn damage(&self) -> usize {
        10
    }
}
