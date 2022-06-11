use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::collision::*;
use crate::game::{BOLT_RADIUS, PELLET_RADIUS, ROCKET_RADIUS, SLUG_RADIUS};

// TODO: pull projectile parameters from a config

pub const PELLET_SPREAD: f32 = 7.0;
pub const NUMBER_OF_PELLETS: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum Projectile {
    Bolt(Entity),
    Pellet(Entity),
    Rocket(Entity),
    Slug(Entity),
}

impl Projectile {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Bolt(_) => "Bolt",
            Self::Pellet(_) => "Pellet",
            Self::Rocket(_) => "Rocket",
            Self::Slug(_) => "Slug",
        }
    }

    pub fn get_owner(&self) -> Entity {
        match self {
            Self::Bolt(owner) => *owner,
            Self::Pellet(owner) => *owner,
            Self::Rocket(owner) => *owner,
            Self::Slug(owner) => *owner,
        }
    }

    pub fn get_mass(&self) -> f32 {
        match self {
            Self::Bolt(_) => 1.0,
            Self::Pellet(_) => 1.0,
            Self::Rocket(_) => 1.0,
            Self::Slug(_) => 1.0,
        }
    }

    pub fn get_initial_speed(&self) -> f32 {
        match self {
            Self::Bolt(_) => 50.0,
            Self::Pellet(_) => 100.0,
            Self::Rocket(_) => 25.0,
            Self::Slug(_) => 150.0,
        }
    }

    pub fn get_bounds(&self) -> Bounds {
        // TODO: not all of these shapes are correct
        match self {
            Self::Bolt(_) => Bounds::Circle(Vec2::ZERO, BOLT_RADIUS),
            Self::Pellet(_) => Bounds::Circle(Vec2::ZERO, PELLET_RADIUS),
            Self::Rocket(_) => Bounds::Circle(Vec2::ZERO, ROCKET_RADIUS),
            Self::Slug(_) => Bounds::Circle(Vec2::ZERO, SLUG_RADIUS),
        }
    }

    pub fn get_damage(&self) -> usize {
        match self {
            Self::Bolt(_) => 1,
            Self::Pellet(_) => 1,
            Self::Rocket(_) => 10,
            Self::Slug(_) => 10,
        }
    }

    pub fn spawn_model(&self, commands: &mut EntityCommands) {
        match self {
            Self::Bolt(_) => {
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
            Self::Pellet(_) => {
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
            Self::Rocket(_) => {
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
            Self::Slug(_) => {
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
