use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::game::PHYSICS_STEP;

#[derive(Debug, Component, Inspectable)]
pub struct Physical {
    pub acceleration: Vec2,
    pub velocity: Vec2,

    pub mass: f32,
}

impl Default for Physical {
    fn default() -> Self {
        Self {
            acceleration: Vec2::default(),
            velocity: Vec2::default(),

            mass: 1.0,
        }
    }
}

impl Physical {
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }

    pub fn teleport(&mut self, transform: &mut Transform, position: Vec2) {
        transform.translation = position.extend(transform.translation.z);

        self.acceleration = Vec2::ZERO;
        self.velocity = Vec2::ZERO;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        self.acceleration += force;
    }

    pub fn update(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        // semi-implicit euler integration
        self.velocity += self.acceleration * dt;
        self.velocity = self.velocity;

        transform.translation += (self.velocity * dt).extend(0.0);

        self.acceleration = Vec2::ZERO;
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct PhysicalQuery<'w> {
    pub transform: &'w Transform,
    pub physical: &'w Physical,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct PhysicalQueryMut<'w> {
    pub transform: &'w Transform,
    pub physical: &'w mut Physical,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct PhysicalQueryUpdateMut<'w> {
    pub transform: &'w mut Transform,
    pub physical: &'w mut Physical,
}
