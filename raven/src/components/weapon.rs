use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::projectile::*;
use crate::components::projectile::*;

// TODO: pull weapon parameters from a config

// TODO: weapon cooldown

const BOLT_SPEED: f32 = 50.0;
const PELLET_SPEED: f32 = 25.0;
const ROCKET_SPEED: f32 = 10.0;
const SLUG_SPEED: f32 = 100.0;

#[derive(Debug, Component, Inspectable)]
pub enum Weapon {
    Blaster,
    Shotgun(usize),
    RocketLauncher(usize),
    Railgun(usize),
}

impl Weapon {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Blaster => "Blaster",
            Self::Shotgun(_) => "Shotgun",
            Self::RocketLauncher(_) => "Rocket Launcher",
            Self::Railgun(_) => "Railgun",
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Blaster => false,
            Self::Shotgun(ammo) => *ammo < 1,
            Self::RocketLauncher(ammo) => *ammo < 1,
            Self::Railgun(ammo) => *ammo < 1,
        }
    }

    pub fn fire(&mut self, commands: &mut Commands, position: Vec2, direction: Vec2) {
        if self.is_empty() {
            return;
        }

        match self {
            Self::Blaster => {
                // TODO: offset
                ProjectileBundle::<Bolt>::spawn_at_position(
                    commands,
                    position,
                    direction * BOLT_SPEED,
                );
                ProjectileBundle::<Bolt>::spawn_at_position(
                    commands,
                    position,
                    direction * BOLT_SPEED,
                );
                ProjectileBundle::<Bolt>::spawn_at_position(
                    commands,
                    position,
                    direction * BOLT_SPEED,
                );
            }
            Self::Shotgun(ammo) => {
                // TODO: spread
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::<Pellet>::spawn_at_position(
                    commands,
                    position,
                    direction * PELLET_SPEED,
                );

                *ammo -= 1;
            }
            Self::RocketLauncher(ammo) => {
                ProjectileBundle::<Rocket>::spawn_at_position(
                    commands,
                    position,
                    direction * ROCKET_SPEED,
                );

                *ammo -= 1;
            }
            Self::Railgun(ammo) => {
                ProjectileBundle::<Slug>::spawn_at_position(
                    commands,
                    position,
                    direction * SLUG_SPEED,
                );

                *ammo -= 1;
            }
        }
    }
}
