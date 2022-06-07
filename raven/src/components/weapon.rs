use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::projectile::*;
use crate::components::inventory::*;
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
}

impl Default for EquippedWeapon {
    fn default() -> Self {
        Self {
            weapon: Weapon::Blaster,
        }
    }
}

impl EquippedWeapon {
    pub fn get_ammo_amount(&self, inventory: &Inventory) -> usize {
        if self.weapon == Weapon::Blaster {
            return 0;
        }
        inventory.get_ammo_amount(self.weapon)
    }

    pub fn is_empty(&self, inventory: &Inventory) -> bool {
        if self.weapon == Weapon::Blaster {
            return false;
        }
        self.get_ammo_amount(inventory) < 1
    }

    pub fn select(&mut self, inventory: &Inventory, weapon: Weapon, name: impl AsRef<str>) {
        if !inventory.has_weapon(weapon) {
            warn!(
                "[{}]: weapon '{}' not in inventory!",
                name.as_ref(),
                weapon.get_name()
            );
            return;
        }

        self.weapon = weapon;
    }

    pub fn fire(
        &mut self,
        commands: &mut Commands,
        inventory: &mut Inventory,
        position: Vec2,
        direction: Vec2,
    ) {
        if self.is_empty(inventory) {
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
            }
            Weapon::RocketLauncher => {
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Rocket,
                    position,
                    direction * ROCKET_SPEED,
                );
            }
            Weapon::Railgun => {
                ProjectileBundle::spawn_at_position(
                    commands,
                    Projectile::Slug,
                    position,
                    direction * SLUG_SPEED,
                );
            }
        }

        inventory.decrease_ammo(self.weapon, 1);
    }
}
