use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::goal::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::game::team::TeamColor;
use crate::game::GOAL_BAR_WIDTH;
use crate::resources::pitch::Pitch;
use crate::resources::SimulationParams;
use crate::{DEBUG_RADIUS, DEBUG_SORT, GOAL_SORT};

#[derive(Debug, Default, Bundle)]
pub struct GoalBundle<T>
where
    T: TeamColorMarker,
{
    pub goal: Goal,
    pub team: T,

    pub bounds: BoundingRect,

    #[bundle]
    pub transform: TransformBundle,
}

impl<T> GoalBundle<T>
where
    T: TeamColorMarker,
{
    pub fn spawn(
        commands: &mut Commands,
        params: &SimulationParams,
        team: T,
        pitch: &Pitch,
    ) -> Entity {
        let team_color = team.team_color();

        let goal_half_extents = params.goal_extents * 0.5;
        let hw = pitch.extents.x * 0.5 - goal_half_extents.x;
        let (position, sign) = match team_color {
            TeamColor::Red => (Vec2::new(-hw, 0.0), 1.0),
            TeamColor::Blue => (Vec2::new(hw, 0.0), -1.0),
        };

        info!("spawning goal for team {:?} at {}", team_color, position);

        let score_center = Vec2::new(sign * goal_half_extents.x, 0.0);
        let top = Vec2::new(score_center.x, score_center.y + goal_half_extents.y);
        let bottom = Vec2::new(score_center.x, score_center.y - goal_half_extents.y);

        info!(
            "goal scoring points for team {:?}: center={}, top={}, bottom={}",
            team_color, score_center, top, bottom
        );

        let mut bundle = commands.spawn_bundle(GoalBundle {
            goal: Goal {
                facing: sign * Vec2::X,

                top,
                bottom,
                score_center,
                ..Default::default()
            },
            team,
            bounds: BoundingRect {
                rect: Rect {
                    left: score_center.x - goal_half_extents.x,
                    right: score_center.x + goal_half_extents.x,
                    top: score_center.y + goal_half_extents.y,
                    bottom: score_center.y - goal_half_extents.y,
                },
            },
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(GOAL_SORT),
            )),
        });

        bundle.insert(Name::new(format!("{:?} Goal", team_color)));

        if params.debug_vis {
            bundle.with_children(|parent| {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            radius: DEBUG_RADIUS,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: Color::PINK,
                            options: FillOptions::default(),
                        }),
                        Transform::from_translation(score_center.extend(DEBUG_SORT)),
                    ))
                    .insert(GoalDebug);

                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            radius: DEBUG_RADIUS,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: Color::ORANGE,
                            options: FillOptions::default(),
                        }),
                        Transform::from_translation(top.extend(DEBUG_SORT)),
                    ))
                    .insert(GoalDebug);

                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            radius: DEBUG_RADIUS,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: Color::OLIVE,
                            options: FillOptions::default(),
                        }),
                        Transform::from_translation(bottom.extend(DEBUG_SORT)),
                    ))
                    .insert(GoalDebug);
            });
        }

        let color = match team_color {
            TeamColor::Red => Color::SALMON,
            TeamColor::Blue => Color::TURQUOISE,
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
