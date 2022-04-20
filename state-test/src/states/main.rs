use bevy::prelude::*;

use crate::bundles::state_test::*;

pub fn setup(mut commands: Commands) {
    StateTestBundle::spawn(&mut commands);
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
