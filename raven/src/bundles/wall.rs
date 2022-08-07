use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::wall::*;
use crate::components::*;
use crate::game::WALL_SORT;

#[derive(Debug, Bundle)]
pub struct WallBundle {
    #[bundle]
    pub spatial: SpatialBundle,

    pub name: Name,

    pub wall: Wall,
}

impl WallBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, from: Vec2, to: Vec2) -> Entity {
        info!("spawning wall at {} ({} to {})", position, from, to);

        let mut bundle = commands.spawn_bundle(WallBundle {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                position.extend(WALL_SORT),
            )),
            name: Name::new("Wall"),
            wall: Wall::new(from, to),
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(from, to),
                    DrawMode::Stroke(StrokeMode {
                        color: Color::GRAY,
                        options: StrokeOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Model)
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
