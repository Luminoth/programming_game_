use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::resources::SimulationParams;

// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

#[derive(Debug, Component, Inspectable)]
pub struct Physical {
    pub acceleration: Vec2,
    pub velocity: Vec2,

    // local coordinate system
    pub heading: Vec2,
    // TODO: smoothed heading
    pub side: Vec2,

    pub mass: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub max_turn_rate: f32,
}

impl Default for Physical {
    fn default() -> Self {
        let heading = Vec2::new(0.0, 1.0);
        let side = heading.perp();

        Self {
            acceleration: Vec2::default(),
            velocity: Vec2::default(),
            heading,
            side,

            mass: 1.0,
            max_speed: 1.0,
            max_force: 1.0,
            max_turn_rate: std::f32::consts::PI,
        }
    }
}

impl Physical {
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }

    pub fn track(&mut self, position: Vec2) {
        // TODO: RotateHeadingToFacePosition(position);
    }

    pub fn future_position(
        &self,
        params: &SimulationParams,
        transform: &Transform,
        dt: f32,
    ) -> Vec2 {
        // x = ut + 1/2(-a)t^2
        // x = distance, a = friction, u = starting velocity

        let ut = self.velocity * dt;
        let half_a_t_squared = 0.5 * -params.friction * dt * dt;

        let scalar_to_vector = half_a_t_squared * self.velocity.normalize_or_zero();

        transform.translation.truncate() + ut + scalar_to_vector
    }

    #[allow(dead_code)]
    pub fn time_to_cover_distance(
        &self,
        params: &SimulationParams,
        a: Vec2,
        b: Vec2,
        force: f32,
    ) -> f32 {
        let speed = force / self.mass;

        // v^2 = u^2 + 2(-a)x

        let distance = a.distance(b);
        let term = speed * speed + 2.0 * distance * -params.friction;

        // if u^2 + 2(-a)x is negative, then we can't reach point b
        if term <= 0.0 {
            return -1.0;
        }

        let v = term.sqrt();

        // t = (v-u) / a
        (v - speed) / -params.friction
    }

    pub fn apply_force(&mut self, force: Vec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        self.acceleration += force.clamp_length_max(self.max_force);
    }

    pub fn update(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        // semi-implicit euler integration
        self.velocity += self.acceleration * dt;
        self.velocity = self.velocity.clamp_length_max(self.max_speed);

        transform.translation += (self.velocity * dt).extend(0.0);

        // update local coordinate system
        if self.velocity.length_squared() > f32::EPSILON {
            self.heading = self.velocity.normalize_or_zero();
            self.side = self.heading.perp();
        }

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
