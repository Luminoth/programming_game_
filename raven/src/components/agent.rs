use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component)]
pub struct SelectedAgent;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Agent;

impl Agent {
    pub fn select(
        &self,
        commands: &mut Commands,
        entity: Entity,
        previous_selected: Option<Entity>,
    ) {
        if let Some(previous_selected) = previous_selected {
            // nothing to do if we are the currently selected agent
            if previous_selected == entity {
                return;
            }

            commands.entity(previous_selected).remove::<SelectedAgent>();
        }
        commands.entity(entity).insert(SelectedAgent);
    }
}
