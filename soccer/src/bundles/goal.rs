use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::goal::*;
use crate::game::{team::Team, GOAL_BAR_WIDTH};
use crate::resources::SimulationParams;
use crate::GOAL_SORT;

#[derive(Debug, Default, Bundle)]
pub struct GoalBundle {
    pub goal: Goal,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl GoalBundle {
    pub fn spawn(commands: &mut Commands, params: &SimulationParams, team: Team) -> Entity {
        info!("spawning goal for team {:?}", team);

        let goal_half_extents = params.goal_extents * 0.5;
        let hw = params.pitch_extents.x * 0.5 - goal_half_extents.x;
        let (position, sign) = match team {
            Team::Red => (Vec2::new(-hw, 0.0), 1.0),
            Team::Blue => (Vec2::new(hw, 0.0), -1.0),
        };

        let score_center = Vec2::new(sign * goal_half_extents.x, 0.0);
        let top = Vec2::new(score_center.x, score_center.y + goal_half_extents.y);
        let bottom = Vec2::new(score_center.x, score_center.y - goal_half_extents.y);

        let mut bundle = commands.spawn_bundle(GoalBundle {
            goal: Goal {
                team,
                facing: sign * Vec2::X,

                top,
                bottom,
                score_center,
            },
            transform: Transform::from_translation(position.extend(GOAL_SORT)),
            ..Default::default()
        });

        bundle.insert(Name::new(format!("{:?} Goal", team)));

        /*bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: 10.0,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::PINK,
                        options: FillOptions::default(),
                    }),
                    Transform::from_translation(score_center.extend(100.0)),
                ))
                .insert(GoalDebug);

            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: 10.0,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::ORANGE,
                        options: FillOptions::default(),
                    }),
                    Transform::from_translation(top.extend(100.0)),
                ))
                .insert(GoalDebug);

            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: 10.0,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::OLIVE,
                        options: FillOptions::default(),
                    }),
                    Transform::from_translation(bottom.extend(100.0)),
                ))
                .insert(GoalDebug);
        });*/

        let color = match team {
            Team::Red => Color::SALMON,
            Team::Blue => Color::TURQUOISE,
        };

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents: params.goal_extents,
                        ..Default::default()
                    },
                    // TODO: we don't want a fill color ...
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(color),
                        outline_mode: StrokeMode::new(Color::GRAY, GOAL_BAR_WIDTH),
                    },
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
