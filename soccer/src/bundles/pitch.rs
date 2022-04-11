use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::obstacle::Wall;
use crate::components::pitch::*;

#[derive(Debug, Default, Bundle)]
struct PitchBorderBundle {
    pub border: PitchBorder,
    pub wall: Wall,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PitchBorderBundle {
    fn spawn(parent: &mut ChildBuilder, position: Vec2, extents: Vec2, name: impl AsRef<str>) {
        parent
            .spawn_bundle(PitchBorderBundle {
                border: PitchBorder::default(),
                wall: Wall::default(),
                transform: Transform::from_translation(position.extend(1.0)),
                ..Default::default()
            })
            .insert(Name::new(format!("{} Border", name.as_ref())))
            .with_children(|parent| {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Rectangle {
                            extents,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: Color::DARK_GRAY,
                            options: FillOptions::default(),
                        }),
                        Transform::default(),
                    ))
                    .insert(Name::new("Model"));
            });
    }
}

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
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
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

        let hh = extents.y * 0.5;
        let hw = extents.x * 0.5;

        bundle.with_children(|parent| {
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(-hw, 0.0),
                Vec2::new(5.0, extents.y),
                "West",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(0.0, hh),
                Vec2::new(extents.x, 5.0),
                "North",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(hw, 0.0),
                Vec2::new(5.0, extents.y),
                "East",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(0.0, -hh),
                Vec2::new(extents.x, 5.0),
                "South",
            );
        });

        bundle.id()
    }
}
