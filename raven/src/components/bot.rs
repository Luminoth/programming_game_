use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::corpse::*;
use crate::components::inventory::*;
use crate::components::weapon::*;

// TODO: pull bot parameters from a config

#[derive(Debug, Component, Inspectable)]
pub struct Bot {
    pub color: Color,

    // TODO: these may need to go in a separate component
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

    fn do_select(&self, commands: &mut Commands, entity: Entity, name: impl AsRef<str>) {
        info!("[{}]: selected!", name.as_ref());

        commands.entity(entity).insert(SelectedBot);
    }

    fn do_possess(&self, commands: &mut Commands, entity: Entity, name: impl AsRef<str>) {
        info!("[{}]: possessed!", name.as_ref());

        commands.entity(entity).insert(PossessedBot);
    }

    pub fn select(
        &self,
        commands: &mut Commands,
        entity: Entity,
        name: impl AsRef<str>,
        previous_selected: Option<(Entity, &Bot, &Name)>,
        previous_possessed: Option<Entity>,
    ) {
        if !self.is_alive() {
            return;
        }

        if let Some((previous_selected_entity, previous_selected_bot, previous_selected_name)) =
            previous_selected
        {
            if previous_selected_entity != entity {
                previous_selected_bot.deselect(
                    commands,
                    previous_selected_entity,
                    previous_selected_name,
                );

                self.do_select(commands, entity, name);
            } else {
                if let Some(previous_possessed) = previous_possessed {
                    if previous_possessed == entity {
                        return;
                    }
                }

                self.do_possess(commands, entity, name);
            }
        } else {
            self.do_select(commands, entity, name);
        }
    }

    pub fn deselect(&self, commands: &mut Commands, entity: Entity, name: &Name) {
        info!("[{}]: released!", name.as_ref());

        commands
            .entity(entity)
            .remove::<SelectedBot>()
            .remove::<PossessedBot>();
    }

    pub fn fire_weapon(
        &self,
        commands: &mut Commands,
        weapon: &mut EquippedWeapon,
        target: Vec2,
        transform: &Transform,
        name: impl AsRef<str>,
    ) {
        if weapon.is_empty() {
            warn!(
                "[{}]: weapon '{}' empty!",
                name.as_ref(),
                weapon.weapon.get_name()
            );
            return;
        }

        let position = transform.translation.truncate();

        info!(
            "[{}]: firing weapon '{}' at {} from {}!",
            name.as_ref(),
            weapon.weapon.get_name(),
            target,
            position
        );

        weapon.fire(commands, position, (target - position).normalize_or_zero());
    }

    pub fn damage(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
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
            self.kill(commands, entity, transform, name);
            return;
        }

        self.current_health -= amount;
    }

    pub fn kill(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        transform: &Transform,
        name: impl AsRef<str>,
    ) {
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

        self.respawn(commands, entity, name);
    }

    pub fn respawn(&mut self, commands: &mut Commands, entity: Entity, name: impl AsRef<str>) {
        if self.is_alive() {
            warn!("[{}] re-alived!", name.as_ref());
        }

        self.current_health = self.max_health;

        commands
            .entity(entity)
            .insert(Inventory::default())
            .insert(EquippedWeapon::default());

        warn!("TODO: respawn {}", name.as_ref());
    }
}

#[derive(Debug, Default, Component)]
pub struct SelectedBot;

// TODO: add support for this
#[derive(Debug, Default, Component)]
pub struct PossessedBot;
