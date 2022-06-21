use bevy::prelude::*;

use crate::components::corpse::*;

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut corpses: Query<(Entity, &mut Corpse, &Name)>,
) {
    for (entity, mut corpse, name) in corpses.iter_mut() {
        corpse.timer.tick(time.delta());

        if corpse.timer.just_finished() {
            info!("despawning corpse '{}'", name);

            commands.entity(entity).despawn_recursive();
        }
    }
}
