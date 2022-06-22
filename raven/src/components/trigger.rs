use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::inventory::*;
use crate::game::cooldown::*;
use crate::game::weapons::*;

// TODO: pull trigger parameters from a config

#[derive(Debug, Component)]
pub enum Trigger {
    Weapon(Weapon, Cooldown),
    Health(Cooldown),
}

impl Trigger {
    pub fn trigger(&mut self, bot: &mut Bot, inventory: &mut Inventory) {
        match self {
            Self::Weapon(weapon, cooldown) => {
                if !cooldown.is_available() {
                    return;
                }

                inventory.increase_ammo(*weapon, weapon.get_ammo().get_trigger_amount());

                cooldown.start();
            }
            Self::Health(cooldown) => {
                if !cooldown.is_available() {
                    return;
                }

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
