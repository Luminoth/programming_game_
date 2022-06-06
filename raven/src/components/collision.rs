use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Clone, Component, Inspectable)]
pub enum Bounds {
    Circle(Vec2, f32),
}

impl Bounds {
    pub fn contains(&self, transform: &Transform, point: Vec2) -> bool {
        match self {
            Self::Circle(center, radius) => {
                let center = transform.translation.truncate() + *center;

                let d = center.distance_squared(point);
                d < radius * radius
            }
        }
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct BoundsQuery<'w> {
    pub bounds: &'w Bounds,
    pub transform: &'w Transform,
}
