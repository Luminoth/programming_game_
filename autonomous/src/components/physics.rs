use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
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

impl Physical {
    pub fn new(mass: f32, max_speed: f32, max_force: f32, max_turn_rate: f32) -> Self {
        Self {
            acceleration: Vec2::default(),
            velocity: Vec2::default(),
            heading: Vec2::default(),
            side: Vec2::default(),
            mass,
            max_speed,
            max_force,
            max_turn_rate,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        self.acceleration += force;
    }

    pub fn update(&mut self, transform: &mut Transform, dt: f32) {
        // semi-implicit euler integration
        self.velocity += self.acceleration * dt;
        self.velocity = self.velocity.clamp_length_max(self.max_speed);

        transform.translation += (self.velocity * dt).extend(0.0);

        // update local coordinate system
        if self.velocity.length_squared() > f32::EPSILON {
            self.heading = self.velocity.normalize();
            self.side = self.heading.perp();
        }

        self.acceleration = Vec2::ZERO;
    }
}
