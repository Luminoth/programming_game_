use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::goal::*;
use crate::game::{Team, GOAL_BAR_WIDTH};

#[derive(Debug, Default, Bundle)]
pub struct GoalBundle {
    pub goal: Goal,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl GoalBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, extents: Vec2, team: Team) -> Entity {
        info!("spawning goal for team {:?} at {}", team, position);

        let mut bundle = commands.spawn_bundle(GoalBundle {
            goal: Goal { team },
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        });

        bundle.insert(Name::new(format!("{:?} Goal", team)));

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents,
                        ..Default::default()
                    },
                    // TODO: we don't want a fill color ...
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::GRAY),
                        outline_mode: StrokeMode::new(Color::GRAY, GOAL_BAR_WIDTH),
                    },
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
