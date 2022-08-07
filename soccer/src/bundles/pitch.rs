use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::obstacle::Wall;
use crate::components::pitch::*;
use crate::game::BORDER_WIDTH;
use crate::resources::pitch::Pitch;
use crate::resources::SimulationParams;
use crate::{BORDER_SORT, DEBUG_RADIUS, DEBUG_SORT, PITCH_SORT};

#[derive(Debug, Default, Bundle)]
struct PitchBorderBundle {
    pub border: PitchBorder,
    pub wall: Wall,

    #[bundle]
    pub spatial: SpatialBundle,
}

impl PitchBorderBundle {
    fn spawn(
        parent: &mut ChildBuilder,
        position: Vec2,
        extents: Vec2,
        facing: Vec2,
        name: impl AsRef<str>,
    ) {
        parent
            .spawn_bundle(PitchBorderBundle {
                border: PitchBorder::default(),
                wall: Wall { extents, facing },
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    position.extend(BORDER_SORT),
                )),
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

#[derive(Debug, Bundle)]
pub struct PitchBundle {
    #[bundle]
    pub spatial: SpatialBundle,
}

impl PitchBundle {
    pub fn spawn(commands: &mut Commands, params: &SimulationParams, pitch: &Pitch) -> Entity {
        info!("spawning pitch");

        let debug_pitch_regions = if params.debug_vis {
            Some(pitch.regions.clone())
        } else {
            None
        };

        let mut bundle = commands.spawn_bundle(PitchBundle {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                Vec2::ZERO.extend(PITCH_SORT),
            )),
        });

        bundle.insert(Name::new("Pitch"));

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents: pitch.extents,
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

        if params.debug_vis {
            bundle.with_children(|parent| {
                for region in debug_pitch_regions.unwrap() {
                    parent
                        .spawn_bundle(GeometryBuilder::build_as(
                            &shapes::Circle {
                                radius: DEBUG_RADIUS * 3.0,
                                ..Default::default()
                            },
                            DrawMode::Fill(FillMode {
                                color: Color::BLACK,
                                options: FillOptions::default(),
                            }),
                            Transform::from_translation(region.position.extend(DEBUG_SORT)),
                        ))
                        .insert(PitchRegionDebug);
                }
            });
        }

        let hs = pitch.extents * 0.5;
        bundle.with_children(|parent| {
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(-hs.x, 0.0),
                Vec2::new(BORDER_WIDTH, pitch.extents.y),
                Vec2::X,
                "West",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(0.0, hs.y),
                Vec2::new(pitch.extents.x, BORDER_WIDTH),
                -Vec2::Y,
                "North",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(hs.x, 0.0),
                Vec2::new(BORDER_WIDTH, pitch.extents.y),
                -Vec2::X,
                "East",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(0.0, -hs.y),
                Vec2::new(pitch.extents.x, BORDER_WIDTH),
                Vec2::Y,
                "South",
            );
        });

        bundle.id()
    }
}
