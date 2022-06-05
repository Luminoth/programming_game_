use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::actor::*;
use crate::components::corpse::*;
use crate::game::{CORPSE_RADIUS, CORPSE_SORT};

#[derive(Debug, Default, Bundle)]
pub struct CorpseBundle {
    #[bundle]
    pub actor: ActorBundle,

    pub corpse: Corpse,
}

impl CorpseBundle {
    pub fn spawn(
        commands: &mut Commands,
        name: impl Into<String>,
        color: Color,
        position: Vec2,
    ) -> Entity {
        let name = name.into();
        info!("spawning corpse '{}' at {}", name, position);

        let mut bundle = commands.spawn_bundle(CorpseBundle {
            actor: ActorBundle {
                name: Name::new(name),
                transform: TransformBundle::from_transform(Transform::from_translation(
                    position.extend(CORPSE_SORT),
                )),
                ..Default::default()
            },
            corpse: Corpse::default(),
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: CORPSE_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
