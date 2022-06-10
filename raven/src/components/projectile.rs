use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::{BOLT_RADIUS, PELLET_RADIUS, ROCKET_RADIUS, SLUG_RADIUS};

// TODO: pull projectile parameters from a config

pub const PELLET_SPREAD: f32 = 7.0;
pub const NUMBER_OF_PELLETS: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Component, Inspectable)]
pub enum Projectile {
    Bolt,
    Pellet,
    Rocket,
    Slug,
}

impl Projectile {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Bolt => "Bolt",
            Self::Pellet => "Pellet",
            Self::Rocket => "Rocket",
            Self::Slug => "Slug",
        }
    }

    pub fn get_mass(&self) -> f32 {
        match self {
            Self::Bolt => 1.0,
            Self::Pellet => 1.0,
            Self::Rocket => 1.0,
            Self::Slug => 1.0,
        }
    }

    pub fn get_initial_speed(&self) -> f32 {
        match self {
            Self::Bolt => 50.0,
            Self::Pellet => 100.0,
            Self::Rocket => 25.0,
            Self::Slug => 150.0,
        }
    }

    pub fn get_damage(&self) -> usize {
        match self {
            Self::Bolt => 1,
            Self::Pellet => 1,
            Self::Rocket => 10,
            Self::Slug => 10,
        }
    }

    pub fn spawn_model(&self, commands: &mut EntityCommands) {
        match self {
            Self::Bolt => {
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
            Self::Pellet => {
                commands.insert_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: PELLET_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::GRAY,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ));
            }
            Self::Rocket => {
                commands.insert_bundle(GeometryBuilder::build_as(
                    // TODO: this is the wrong shape for a rocket
                    &shapes::Circle {
                        radius: ROCKET_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::ORANGE_RED,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ));
            }
            Self::Slug => {
                commands.insert_bundle(GeometryBuilder::build_as(
                    // TODO: this is the wrong shape for a slug
                    &shapes::Circle {
                        radius: SLUG_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::PURPLE,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ));
            }
        }
    }
}
