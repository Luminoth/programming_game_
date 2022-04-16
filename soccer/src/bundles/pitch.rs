use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::obstacle::Wall;
use crate::components::pitch::*;
use crate::game::BORDER_WIDTH;
use crate::resources::SimulationParams;
use crate::{BORDER_SORT, PITCH_SORT};

#[derive(Debug, Default, Bundle)]
struct PitchBorderBundle {
    pub border: PitchBorder,
    pub wall: Wall,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
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
                transform: Transform::from_translation(position.extend(BORDER_SORT)),
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

#[derive(Debug, Bundle)]
pub struct PitchBundle {
    pub pitch: Pitch,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PitchBundle {
    pub fn spawn(commands: &mut Commands, params: &SimulationParams) -> Entity {
        info!("spawning pitch");

        let mut bundle = commands.spawn_bundle(PitchBundle {
            pitch: Pitch::new(params),
            transform: Transform::from_translation(Vec2::ZERO.extend(PITCH_SORT)),
            global_transform: GlobalTransform::default(),
        });

        bundle.insert(Name::new("Pitch"));

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents: params.pitch_extents,
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

        let hh = params.pitch_extents.y * 0.5;
        let hw = params.pitch_extents.x * 0.5;

        bundle.with_children(|parent| {
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(-hw, 0.0),
                Vec2::new(BORDER_WIDTH, params.pitch_extents.y),
                Vec2::X,
                "West",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(0.0, hh),
                Vec2::new(params.pitch_extents.x, BORDER_WIDTH),
                -Vec2::Y,
                "North",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(hw, 0.0),
                Vec2::new(BORDER_WIDTH, params.pitch_extents.y),
                -Vec2::X,
                "East",
            );
            PitchBorderBundle::spawn(
                parent,
                Vec2::new(0.0, -hh),
                Vec2::new(params.pitch_extents.x, BORDER_WIDTH),
                Vec2::Y,
                "South",
            );
        });

        bundle.id()
    }
}
