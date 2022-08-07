use bevy::prelude::*;

use crate::components::spawnpoint::*;

#[derive(Debug, Bundle)]
pub struct SpawnPointBundle {
    #[bundle]
    pub spatial: SpatialBundle,

    pub name: Name,

    pub spawnpoint: SpawnPoint,
}

impl SpawnPointBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, offset: Vec2) -> Entity {
        info!("spawning spawnpoint at {}", position,);

        let bundle = commands.spawn_bundle(SpawnPointBundle {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            name: Name::new("Spawnpoint"),
            spawnpoint: SpawnPoint { offset },
        });

        bundle.id()
    }
}
