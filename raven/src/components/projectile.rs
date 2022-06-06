use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::BOLT_RADIUS;

// TODO: pull projectile parameters from a config

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
            Self::Pellet => todo!(),
            Self::Rocket => todo!(),
            Self::Slug => todo!(),
        }
    }
}
