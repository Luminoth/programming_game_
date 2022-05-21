use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::steering::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Agent;

impl Agent {
    pub fn separation_on(commands: &mut EntityCommands) {
        commands.insert(SoccerPlayerSeparation);
    }

    #[allow(dead_code)]
    pub fn separation_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<SoccerPlayerSeparation>();
    }

    pub fn seek_on(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(Seek);
    }

    pub fn seek_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Seek>();
    }

    pub fn arrive_on(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(Arrive::default());
    }

    pub fn arrive_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Arrive>();
    }

    pub fn pursuit_on(&self, commands: &mut Commands, entity: Entity, target: Entity) {
        commands.entity(entity).insert(Pursuit { target });
    }

    pub fn pursuit_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Pursuit>();
    }

    pub fn interpose_on(
        &self,
        commands: &mut Commands,
        entity: Entity,
        target: Entity,
        distance: f32,
    ) {
        commands
            .entity(entity)
            .insert(Interpose { target, distance });
    }

    pub fn interpose_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Interpose>();
    }
}
