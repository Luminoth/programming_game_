use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::corpse::*;

// TODO: pull weapon parameters from a config

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

    pub fn get_health_percent(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }

    pub fn select(
        &self,
        commands: &mut Commands,
        entity: Entity,
        previous_selected: Option<Entity>,
    ) {
        if let Some(previous_selected) = previous_selected {
            // nothing to do if we are the currently selected bot
            if previous_selected == entity {
                return;
            }

            commands.entity(previous_selected).remove::<SelectedBot>();
        }
        commands.entity(entity).insert(SelectedBot);
    }

    pub fn damage(&mut self, amount: usize) -> bool {
        if amount > self.current_health {
            self.current_health = 0;
            return true;
        }

        self.current_health -= amount;
        false
    }

    pub fn kill(&mut self, commands: &mut Commands, transform: &Transform, name: impl AsRef<str>) {
        if self.current_health > 0 {
            warn!("killing bot '{}' who isn't dead!", name.as_ref());
            self.current_health = 0;
        }

        let position = transform.translation.truncate();

        CorpseBundle::spawn(
            commands,
            format!("{} Corpse", name.as_ref()),
            self.color,
            position,
        );
    }

    pub fn respawn(&mut self) {
        self.current_health = self.max_health;

        todo!();
    }
}

#[derive(Debug, Default, Component)]
pub struct SelectedBot;
