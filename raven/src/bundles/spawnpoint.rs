use bevy::prelude::*;

use crate::components::spawnpoint::*;

#[derive(Debug, Bundle)]
pub struct SpawnPointBundle {
    #[bundle]
    pub transform: TransformBundle,

    pub spawnpoint: SpawnPoint,
}

impl SpawnPointBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, offset: Vec2) -> Entity {
        info!("spawning spawnpoint at {}", position,);

        let bundle = commands.spawn_bundle(SpawnPointBundle {
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            spawnpoint: SpawnPoint { offset },
        });

        bundle.id()
    }
}
