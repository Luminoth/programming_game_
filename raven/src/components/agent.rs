use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::components::steering::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Agent;

impl Agent {
    pub fn seek_on(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(Seek);
    }

    pub fn seek_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Seek>();
    }

    pub fn arrive_on(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(Arrive::default());
    }

    pub fn arrive_on_with_deceleration(
        &self,
        commands: &mut Commands,
        entity: Entity,
        deceleration: Deceleration,
    ) {
        commands.entity(entity).insert(Arrive { deceleration });
    }

    pub fn arrive_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Arrive>();
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct AgentQueryMut<'w> {
    pub agent: &'w mut Agent,
    pub steering: &'w mut Steering,
}
