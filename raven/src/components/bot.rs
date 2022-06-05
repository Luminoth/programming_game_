use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::corpse::*;
use crate::components::weapon::*;

// TODO: pull bot parameters from a config

#[derive(Debug, Component, Inspectable)]
pub struct Bot {
    pub color: Color,

    pub max_health: usize,
    pub current_health: usize,
}

impl Bot {
    pub fn new(color: Color, health: usize) -> Self {
        Self {
            color,
            max_health: health,
            current_health: health,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }

    pub fn get_health_percent(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }

    pub fn select(
        &self,
        commands: &mut Commands,
        entity: Entity,
        name: impl AsRef<str>,
        previous_selected: Option<Entity>,
    ) {
        if !self.is_alive() {
            return;
        }

        info!("[{}]: selected!", name.as_ref());

        if let Some(previous_selected) = previous_selected {
            // nothing to do if we are the currently selected bot
            if previous_selected == entity {
                return;
            }

            commands.entity(previous_selected).remove::<SelectedBot>();
        }
        commands.entity(entity).insert(SelectedBot);
    }

    pub fn fire_weapon<T>(
        &self,
        commands: &mut Commands,
        weapon: &mut T,
        target: Vec2,
        transform: &Transform,
        name: impl AsRef<str>,
    ) where
        T: WeaponType,
    {
        if weapon.is_empty() {
            warn!("[{}]: weapon '{}' empty!", name.as_ref(), T::name());
            return;
        }

        let position = transform.translation.truncate();

        info!(
            "[{}]: firing weapon '{}' at {} from {}!",
            name.as_ref(),
            T::name(),
            target,
            position
        );

        weapon.fire(commands, position, (target - position).normalize_or_zero());
    }

    pub fn damage(
        &mut self,
        commands: &mut Commands,
        transform: &Transform,
        name: impl AsRef<str>,
        amount: usize,
    ) {
        if !self.is_alive() {
            warn!("[{}]: attempt to damage while dead!", name.as_ref());
            return;
        }

        info!(
            "[{}]: damaged {} ({})",
            name.as_ref(),
            amount,
            self.current_health
        );

        if amount >= self.current_health {
            self.current_health = 0;
            self.kill(commands, transform, name);
            return;
        }

        self.current_health -= amount;
    }

    pub fn kill(&mut self, commands: &mut Commands, transform: &Transform, name: impl AsRef<str>) {
        if self.is_alive() {
            warn!("[{}] unalived!", name.as_ref());
            self.current_health = 0;
        }

        let position = transform.translation.truncate();

        CorpseBundle::spawn(
            commands,
            format!("{} Corpse", name.as_ref()),
            self.color,
            position,
        );

        self.respawn(name);
    }

    pub fn respawn(&mut self, name: impl AsRef<str>) {
        if self.is_alive() {
            warn!("[{}] re-alived!", name.as_ref());
        }

        self.current_health = self.max_health;

        warn!("TODO: respawn {}", name.as_ref());
    }
}

#[derive(Debug, Default, Component)]
pub struct SelectedBot;