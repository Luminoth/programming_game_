use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::projectile::*;
use crate::bundles::trigger::*;
use crate::components::inventory::*;
use crate::components::projectile::*;
use crate::components::trigger::*;
use crate::game::cooldown::*;
use crate::game::weapons::*;
use crate::game::PHYSICS_STEP;
use crate::util::*;

// TODO: weapon cooldown could be better:
// 1) global cooldown on switching weapons
// 2) set the cooldown on the weapon in the inventory
//    so that it keeps rolling

#[derive(Debug, Component, Inspectable)]
pub struct EquippedWeapon {
    pub weapon: Weapon,

    #[inspectable(ignore)]
    pub cooldown: Cooldown,
}

impl Default for EquippedWeapon {
    fn default() -> Self {
        let weapon = Weapon::Blaster;
        Self {
            weapon,
            cooldown: Cooldown::from_seconds(weapon.get_cooldown_seconds()),
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

    pub fn is_ready(&self) -> bool {
        self.cooldown.is_available()
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

        info!(
            "[{}] selecting weapon '{}'",
            name.as_ref(),
            weapon.get_name()
        );

        self.weapon = weapon;
        self.cooldown
            .set_duration(self.weapon.get_cooldown_seconds());
    }

    pub fn fire(
        &mut self,
        commands: &mut Commands,
        owner: Entity,
        inventory: &mut Inventory,
        position: Vec2,
        direction: Vec2,
    ) {
        if !self.is_ready() || self.is_empty(inventory) {
            return;
        }

        match self.weapon {
            Weapon::Blaster => {
                ProjectileBundle::spawn(commands, Projectile::Bolt(owner), position, direction);
            }
            Weapon::Shotgun => {
                let spread = PELLET_SPREAD.to_radians();
                let stride = spread / NUMBER_OF_PELLETS as f32;

                let mut angle = -spread / 2.0;
                for _ in 0..NUMBER_OF_PELLETS {
                    let direction = direction.rotate_origin(angle);

                    ProjectileBundle::spawn(
                        commands,
                        Projectile::Pellet(owner),
                        position,
                        direction,
                    );

                    angle += stride;
                }
            }
            Weapon::RocketLauncher => {
                ProjectileBundle::spawn(commands, Projectile::Rocket(owner), position, direction);
            }
            Weapon::Railgun => {
                ProjectileBundle::spawn(commands, Projectile::Slug(owner), position, direction);
            }
        }

        inventory.decrease_ammo(self.weapon, 1);

        self.cooldown.start();

        TriggerBundle::spawn(
            commands,
            Trigger::Sound(owner, Timer::from_seconds(PHYSICS_STEP, false)),
            position,
            self.weapon.get_sound_radius(),
        );
    }
}
