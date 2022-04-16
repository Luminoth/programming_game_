use bevy::prelude::*;

use crate::components::miner::*;
use crate::components::wife::*;
use crate::resources::messaging::*;

pub fn setup(mut commands: Commands) {
    // spawn miner / wife entities
    let miner_id = Miner::spawn(&mut commands, "Bob");
    let wife_id = Wife::spawn(&mut commands, "Elsa");

    // pair miners and wives
    commands.entity(miner_id).insert(MinerWife { wife_id });
    commands.entity(wife_id).insert(WifeMiner { miner_id });

    // add resources
    commands.insert_resource(MessageDispatcher::default());
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<MessageDispatcher>();
}
