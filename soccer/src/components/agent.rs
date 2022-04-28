use bevy::prelude::*;

use crate::components::steering::*;

#[derive(Debug, Default, Component)]
pub struct Agent;

impl Agent {
    pub fn seek_on(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(Seek);
    }

    pub fn seek_off(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Seek>();
    }
}
