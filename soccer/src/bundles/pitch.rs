use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::pitch::*;

#[derive(Debug, Default, Bundle)]
pub struct PitchBundle {
    pub pitch: Pitch,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PitchBundle {
    pub fn spawn(commands: &mut Commands, extents: Vec2) -> Entity {
        info!("spawning pitch");

        let mut bundle = commands.spawn_bundle(PitchBundle {
            pitch: Pitch::default(),
            ..Default::default()
        });

        bundle.insert(Name::new("Pitch"));

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::GREEN,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
