use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::inventory::*;
use crate::game::cooldown::*;
use crate::game::weapons::*;

// TODO: pull trigger parameters from a config

#[derive(Debug, strum_macros::Display, Component)]
pub enum Trigger {
    Weapon(Weapon, Cooldown),
    Health(Cooldown),
}

impl Trigger {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Weapon(_, _) => Color::GOLD,
            Self::Health(_) => Color::DARK_GREEN,
        }
    }

    pub fn trigger(&mut self, bot: &mut Bot, inventory: &mut Inventory, name: &Name) {
        match self {
            Self::Weapon(weapon, cooldown) => {
                if !cooldown.is_available() {
                    return;
                }

                let ammo = weapon.get_ammo();

                info!("[{}]: ammo {} pickup!", name.as_ref(), ammo);

                inventory.increase_ammo(ammo, ammo.get_trigger_amount());

                cooldown.start();
            }
            Self::Health(cooldown) => {
                if !cooldown.is_available() {
                    return;
                }

                info!("[{}]: health pickup!", name.as_ref());

                bot.increase_health(5);

                cooldown.start();
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self {
            Self::Weapon(_, cooldown) => {
                cooldown.tick(dt);
            }
            Self::Health(cooldown) => {
                cooldown.tick(dt);
            }
        }
    }
}
