use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::projectile::*;
use crate::components::projectile::*;
use crate::game::weapons::*;

// TODO: pull weapon parameters from a config

// TODO: weapon cooldown

const BOLT_SPEED: f32 = 50.0;
const PELLET_SPEED: f32 = 25.0;
const ROCKET_SPEED: f32 = 10.0;
const SLUG_SPEED: f32 = 100.0;

#[derive(Debug, Component, Inspectable)]
pub struct EquippedWeapon {
    pub weapon: Weapon,
    pub ammo: usize,
}

impl Default for EquippedWeapon {
    fn default() -> Self {
        Self {
            weapon: Weapon::Blaster,
            ammo: 0,
        }
    }
}

impl EquippedWeapon {
    pub fn is_empty(&self) -> bool {
        if self.weapon == Weapon::Blaster {
            return false;
        }
        self.ammo < 1
    }

    pub fn fire(&mut self, commands: &mut Commands, position: Vec2, direction: Vec2) {
        if self.is_empty() {
            return;
        }

        match self.weapon {
            Weapon::Blaster => {
                // TODO: offset
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Bolt,
                    position,
                    direction * BOLT_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Bolt,
                    position,
                    direction * BOLT_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Bolt,
                    position,
                    direction * BOLT_SPEED,
                );
            }
            Weapon::Shotgun => {
                // TODO: spread
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Pellet,
                    position,
                    direction * PELLET_SPEED,
                );

                self.ammo -= 1;
            }
            Weapon::RocketLauncher => {
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Rocket,
                    position,
                    direction * ROCKET_SPEED,
                );

                self.ammo -= 1;
            }
            Weapon::Railgun => {
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Slug,
                    position,
                    direction * SLUG_SPEED,
                );

                self.ammo -= 1;
            }
        }
    }
}
