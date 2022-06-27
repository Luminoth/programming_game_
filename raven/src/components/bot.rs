use bevy::ecs::query::WorldQuery;
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
    pub invulnerable: bool,
}

impl Bot {
    pub fn new(color: Color, health: usize) -> Self {
        Self {
            color,
            max_health: health,
            current_health: health,
            invulnerable: false,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }

    pub fn get_health_percent(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }

    fn do_select(
        commands: &mut Commands,
        entity: Entity,
        name: impl AsRef<str>,
        children: &Children,
        selected_visibility: &mut Query<
            &mut Visibility,
            (With<SelectedBotVisual>, Without<PossessedBotVisual>),
        >,
    ) {
        info!("[{}]: selected!", name.as_ref());

        commands.entity(entity).insert(SelectedBot);

        for &child in children.iter() {
            if let Ok(mut selected_visibility) = selected_visibility.get_mut(child) {
                selected_visibility.is_visible = true;
                break;
            }
        }
    }

    fn do_possess(
        commands: &mut Commands,
        entity: Entity,
        name: impl AsRef<str>,
        children: &Children,
        possessed_visibility: &mut Query<
            &mut Visibility,
            (With<PossessedBotVisual>, Without<SelectedBotVisual>),
        >,
    ) {
        info!("[{}]: possessed!", name.as_ref());

        commands.entity(entity).insert(PossessedBot);

        for &child in children.iter() {
            if let Ok(mut possessed_visibility) = possessed_visibility.get_mut(child) {
                possessed_visibility.is_visible = true;
            }
        }
    }

    pub fn select(
        &self,
        commands: &mut Commands,
        entity: Entity,
        name: impl AsRef<str>,
        children: &Children,
        previous_selected: Option<(Entity, BotQueryItem, &Children)>,
        previous_possessed: Option<Entity>,
        selected_visibility: &mut Query<
            &mut Visibility,
            (With<SelectedBotVisual>, Without<PossessedBotVisual>),
        >,
        possessed_visibility: &mut Query<
            &mut Visibility,
            (With<PossessedBotVisual>, Without<SelectedBotVisual>),
        >,
    ) {
        if !self.is_alive() {
            return;
        }

        if let Some((previous_selected_entity, previous_selected_bot, previous_selected_children)) =
            previous_selected
        {
            if previous_selected_entity != entity {
                previous_selected_bot.bot.deselect(
                    commands,
                    previous_selected_entity,
                    previous_selected_bot.name,
                    previous_selected_children,
                    selected_visibility,
                    possessed_visibility,
                );

                Self::do_select(commands, entity, name, children, selected_visibility);
            } else {
                if let Some(previous_possessed_entity) = previous_possessed {
                    if previous_possessed_entity == entity {
                        return;
                    }
                }

                Self::do_possess(commands, entity, name, children, possessed_visibility);
            }
        } else {
            Self::do_select(commands, entity, name, children, selected_visibility);
        }
    }

    pub fn deselect(
        &self,
        commands: &mut Commands,
        entity: Entity,
        name: impl AsRef<str>,
        children: &Children,
        selected_visibility: &mut Query<
            &mut Visibility,
            (With<SelectedBotVisual>, Without<PossessedBotVisual>),
        >,
        possessed_visibility: &mut Query<
            &mut Visibility,
            (With<PossessedBotVisual>, Without<SelectedBotVisual>),
        >,
    ) {
        info!("[{}]: released!", name.as_ref());

        commands
            .entity(entity)
            .remove::<SelectedBot>()
            .remove::<PossessedBot>();

        for &child in children.iter() {
            if let Ok(mut selected_visibility) = selected_visibility.get_mut(child) {
                selected_visibility.is_visible = false;
            }

            if let Ok(mut possessed_visibility) = possessed_visibility.get_mut(child) {
                possessed_visibility.is_visible = false;
            }
        }
    }

    pub fn fire_weapon(
        &self,
        commands: &mut Commands,
        entity: Entity,
        weapon: &mut EquippedWeapon,
        inventory: &mut Inventory,
        target: Vec2,
        transform: &Transform,
        name: impl AsRef<str>,
    ) {
        if !self.is_alive() {
            warn!("[{}]: can't fire weapon while dead!", name.as_ref(),);
            return;
        }

        if !weapon.is_ready() {
            warn!(
                "[{}]: weapon '{}' not ready!",
                name.as_ref(),
                weapon.weapon.get_name()
            );
            return;
        }

        if weapon.is_empty(inventory) {
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

        weapon.fire(
            commands,
            entity,
            inventory,
            position,
            (target - position).normalize_or_zero(),
        );
    }

    pub fn increase_health(&mut self, amount: usize) {
        let available = self.max_health - self.current_health;
        let amount = available.min(amount);

        if amount > 0 {
            self.current_health += amount;
        }
    }

    pub fn damage(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        transform: &Transform,
        inventory: &mut Inventory,
        name: impl AsRef<str>,
        amount: usize,
    ) {
        if !self.is_alive() {
            warn!("[{}]: attempt to damage while dead!", name.as_ref());
            return;
        }

        if self.invulnerable {
            info!("[{}]: invulnerable!", name.as_ref());
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
            self.kill(commands, entity, transform, inventory, name);
            return;
        }

        self.current_health -= amount;
    }

    pub fn kill(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        transform: &Transform,
        inventory: &mut Inventory,
        name: impl AsRef<str>,
    ) {
        if self.invulnerable {
            warn!("[{}]: unkillable but unalived!", name.as_ref());
        } else if self.is_alive() {
            warn!("[{}] unalived!", name.as_ref());
        }

        self.current_health = 0;

        let position = transform.translation.truncate();

        CorpseBundle::spawn(
            commands,
            format!("{} Corpse", name.as_ref()),
            self.color,
            position,
        );

        self.respawn(commands, entity, inventory, name);
    }

    // TODO: we need to respawn on the *next* frame
    // but we should also have bots be invincible on spawn for a little bit
    pub fn respawn(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        inventory: &mut Inventory,
        name: impl AsRef<str>,
    ) {
        if self.invulnerable {
            warn!("[{}]: unkillable but reborn!", name.as_ref());
        } else if self.is_alive() {
            warn!("[{}] reborn!", name.as_ref());
        }

        self.current_health = self.max_health;
        inventory.reset();

        commands
            .entity(entity)
            .insert(Inventory::default())
            .insert(EquippedWeapon::default());

        warn!("TODO: respawn {}", name.as_ref());
    }
}

// this doesn't include a transform because
// most of the time the PhysicalQuery captures that
#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct BotQuery<'w> {
    pub bot: &'w Bot,
    pub name: &'w Name,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct BotQueryMut<'w> {
    pub bot: &'w mut Bot,
    pub name: &'w Name,
}

#[derive(Debug, Default, Component)]
pub struct SelectedBot;

#[derive(Debug, Default, Component)]
pub struct SelectedBotVisual;

#[derive(Debug, Default, Component)]
pub struct PossessedBot;

#[derive(Debug, Default, Component)]
pub struct PossessedBotVisual;
