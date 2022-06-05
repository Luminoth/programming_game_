use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

pub trait Bounds {
    fn contains(&self, transform: &Transform, point: Vec2) -> bool;
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
