use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

pub trait Bounds: Default + Component {
    fn contains(&self, transform: &Transform, point: Vec2) -> bool;
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct BoundsQuery<'w, T>
where
    T: Bounds,
{
    pub bounds: &'w T,
    pub transform: &'w Transform,
}

#[derive(Debug, Component, Inspectable)]
pub struct BoundingCircle {
    pub center: Vec2,
    pub radius: f32,
}

impl Default for BoundingCircle {
    fn default() -> Self {
        Self {
            center: Vec2::default(),
            radius: 1.0,
        }
    }
}

impl BoundingCircle {
    pub fn from_radius(radius: f32) -> Self {
        Self {
            radius,
            ..Default::default()
        }
    }
}

impl Bounds for BoundingCircle {
    fn contains(&self, transform: &Transform, point: Vec2) -> bool {
        let center = transform.translation.truncate() + self.center;

        let d = center.distance_squared(point);
        d < self.radius * self.radius
    }
}
