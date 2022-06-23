use std::time::Duration;

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
    Sound(Entity, Timer),
}

impl Trigger {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Weapon(_, _) => Color::GOLD,
            Self::Health(_) => Color::DARK_GREEN,
            Self::Sound(_, _) => Color::CRIMSON,
        }
    }

    pub fn trigger(
        &mut self,
        entity: Entity,
        bot: &mut Bot,
        inventory: &mut Inventory,
        name: impl AsRef<str>,
    ) {
        if !bot.is_alive() {
            return;
        }

        match self {
            Self::Weapon(weapon, cooldown) => {
                if !cooldown.is_available() {
                    return;
                }

                let ammo = weapon.get_ammo();

                info!("[{}]: ammo {} pickup!", name.as_ref(), ammo);

                inventory.increase_ammo(*weapon, ammo.get_powerup_amount());

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
            Self::Sound(owner, _) => {
                if entity == *owner {
                    return;
                }

                warn!("[{}] heard a sound!", name.as_ref());
            }
        }
    }

    pub fn update(&mut self, commands: &mut Commands, entity: Entity, dt: f32) {
        match self {
            Self::Weapon(_, cooldown) => {
                cooldown.tick(dt);
            }
            Self::Health(cooldown) => {
                cooldown.tick(dt);
            }
            Self::Sound(_, timer) => {
                timer.tick(Duration::from_secs_f32(dt));

                if timer.just_finished() {
                    info!("despawning Sound trigger");

                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
