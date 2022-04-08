use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

#[derive(Debug, Component, Inspectable)]
pub struct Physical {
    pub acceleration: Vec2,
    pub velocity: Vec2,

    // local coordinate system
    pub heading: Vec2,
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

            // TODO: these defaults aren't good
            mass: 0.0,
            max_speed: 0.0,
            max_force: 0.0,
            max_turn_rate: 0.0,
        }
    }
}

impl Physical {
    pub fn speed(&self) -> f32 {
        self.velocity.length()
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
