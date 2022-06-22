use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::collision::*;
use crate::components::trigger::*;
use crate::components::*;
use crate::game::TRIGGER_RADIUS;

#[derive(Debug, Bundle)]
pub struct TriggerBundle {
    #[bundle]
    pub transform: TransformBundle,

    pub bounds: Bounds,

    pub trigger: Trigger,
}

impl TriggerBundle {
    pub fn spawn(commands: &mut Commands, trigger: Trigger, position: Vec2) -> Entity {
        info!("spawning trigger {} at {}", trigger, position);

        let color = trigger.get_color();

        let mut bundle = commands.spawn_bundle(TriggerBundle {
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            bounds: Bounds::Circle(Vec2::ZERO, TRIGGER_RADIUS),
            trigger,
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: TRIGGER_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color,
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
