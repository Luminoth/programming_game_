use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::game::PHYSICS_STEP;

#[derive(Debug, Default, Inspectable)]
pub struct PhysicalCache {
    pub speed: f32,

    pub position: Vec2,
    pub future_position: Vec2,
    pub heading: Vec2,
    pub max_distance: f32,
}

#[derive(Debug, Component, Inspectable)]
pub struct Physical {
    pub acceleration: Vec2,
    pub velocity: Vec2,

    pub mass: f32,

    pub max_speed: f32,
    pub max_force: f32,
    pub max_turn_rate: f32,

    pub cache: PhysicalCache,
}

impl Default for Physical {
    fn default() -> Self {
        Self {
            acceleration: Vec2::default(),
            velocity: Vec2::default(),

            mass: 1.0,

            max_speed: f32::MAX,
            max_force: f32::MAX,
            max_turn_rate: std::f32::consts::PI,

            cache: PhysicalCache::default(),
        }
    }
}

impl Physical {
    pub fn get_speed(&self) -> f32 {
        self.velocity.length()
    }

    pub fn get_future_position(&self, transform: &Transform, dt: f32) -> Vec2 {
        let position = transform.translation.truncate();
        position + self.velocity * dt
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

        self.acceleration += force.clamp_length_max(self.max_force);
    }

    pub fn stop(&mut self) {
        self.acceleration = Vec2::ZERO;
        self.velocity = Vec2::ZERO;
    }

    pub fn update(&mut self, transform: &Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        // semi-implicit euler integration
        self.velocity += self.acceleration * dt;
        self.velocity = self.velocity.clamp_length_max(self.max_speed);

        self.acceleration = Vec2::ZERO;

        self.cache.speed = self.get_speed();

        self.cache.position = transform.translation.truncate();
        self.cache.future_position = self.get_future_position(transform, dt);

        let heading = self.cache.future_position - self.cache.position;
        self.cache.max_distance = heading.length();
        self.cache.heading = heading.normalize_or_zero();
    }

    pub fn sync(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        transform.translation += (self.velocity * dt).extend(0.0);
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
