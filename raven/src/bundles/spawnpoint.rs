use bevy::prelude::*;

use crate::components::spawnpoint::*;

#[derive(Debug, Bundle)]
pub struct SpawnPointBundle {
    #[bundle]
    pub transform: TransformBundle,

    pub name: Name,

    pub spawnpoint: SpawnPoint,
}

impl SpawnPointBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, offset: Vec2) -> Entity {
        info!("spawning spawnpoint at {}", position,);

        let bundle = commands.spawn_bundle(SpawnPointBundle {
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            name: Name::new("Spawnpoint"),
            spawnpoint: SpawnPoint { offset },
        });

        bundle.id()
    }
}
