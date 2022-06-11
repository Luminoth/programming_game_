use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::wall::*;
use crate::components::*;
use crate::game::WALL_SORT;

#[derive(Debug, Bundle)]
pub struct WallBundle {
    #[bundle]
    pub transform: TransformBundle,

    pub wall: Wall,
}

impl WallBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, extents: Vec2) -> Entity {
        info!("spawning wall at {} ({})", position, extents);

        let mut bundle = commands.spawn_bundle(WallBundle {
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(WALL_SORT),
            )),
            wall: Wall::default(),
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::GRAY,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Model)
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
