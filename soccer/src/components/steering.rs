use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::resources::*;

use super::physics::*;

pub trait SteeringBehavior: std::fmt::Debug + Component {}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct Steering {
    pub target: Vec2,
    pub accumulated_force: Vec2,
}

impl Steering {
    pub fn is_at_target(&self, params: &SimulationParams, transform: &Transform) -> bool {
        transform
            .translation
            .truncate()
            .distance_squared(self.target)
            < params.player_in_target_range_squared
    }

    pub fn accumulate_force(&mut self, physical: &Physical, force: Vec2, weight: f32) {
        let force = force * weight;

        let mag_so_far = self.accumulated_force.length();
        let mag_remain = physical.max_force - mag_so_far;
        if mag_remain <= 0.0 {
            return;
        }

        let mut to_add = force.length();
        if to_add > mag_remain {
            to_add = mag_remain;
        }

        self.accumulated_force += force.normalize_or_zero() * to_add;
    }

    pub fn update(&mut self, physical: &mut Physical) {
        physical.apply_force(self.accumulated_force);
        self.accumulated_force = Vec2::ZERO;
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SteeringQueryMut<'w> {
    pub steering: &'w mut Steering,
    pub physical: &'w mut Physical,
}

#[derive(Debug, Default, Component)]
pub struct SteeringTargetDebug;

fn seek_force(target: Vec2, physical: &PhysicalQueryItem) -> Vec2 {
    let position = physical.transform.translation.truncate();

    let desired_velocity = (target - position).normalize_or_zero() * physical.physical.max_speed;
    desired_velocity - physical.physical.velocity
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct Seek;

impl SteeringBehavior for Seek {}

impl Seek {
    pub fn force(&self, steering: &Steering, physical: &PhysicalQueryItem) -> Vec2 {
        seek_force(steering.target, physical)
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct SeekQueryMut<'w> {
    pub seek: &'w Seek,
    pub steering: &'w mut Steering,
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub enum Deceleration {
    Slow = 3,
    Normal = 2,
    Fast = 1,
}

impl Default for Deceleration {
    fn default() -> Self {
        Self::Normal
    }
}

fn arrive_force(target: Vec2, physical: &PhysicalQueryItem, deceleration: Deceleration) -> Vec2 {
    let position = physical.transform.translation.truncate();
    let deceleration = deceleration as i32;

    let to_target = target - position;

    let dist = to_target.length();
    if dist > 0.0 {
        // fine tweaking of deceleration
        let deceleration_tweaker = 0.3;

        let speed =
            (dist / (deceleration as f32 * deceleration_tweaker)).min(physical.physical.max_speed);
        let desired_velocity = to_target * speed / dist;
        return desired_velocity - physical.physical.velocity;
    }

    Vec2::ZERO
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct Arrive {
    pub deceleration: Deceleration,
}

impl SteeringBehavior for Arrive {}

impl Arrive {
    pub fn force(&self, steering: &Steering, physical: &PhysicalQueryItem) -> Vec2 {
        arrive_force(steering.target, physical, self.deceleration)
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct ArriveQueryMut<'w> {
    pub arrive: &'w Arrive,
    pub steering: &'w mut Steering,
}

#[derive(Debug, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct Pursuit {
    #[inspectable(ignore)]
    pub target: Entity,
}

impl SteeringBehavior for Pursuit {}

impl Pursuit {
    pub fn force(
        &self,
        params: &SimulationParams,
        steering: &Steering,
        physical: &PhysicalQueryItem,
        physicals: &Query<PhysicalQuery>,
    ) -> Vec2 {
        let target = physicals.get(self.target).unwrap();

        let position = physical.transform.translation.truncate();

        let to_target = steering.target - position;

        let mut look_ahead_time = 0.0;
        if target.physical.speed() != 0.0 {
            look_ahead_time = to_target.length() / target.physical.speed();
        }

        let target = target
            .physical
            .future_position(params, target.transform, look_ahead_time);

        arrive_force(target, physical, Deceleration::Fast)
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct PursuitQueryMut<'w> {
    pub pursuit: &'w Pursuit,
    pub steering: &'w mut Steering,
}

#[derive(Debug, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct Interpose {
    pub distance: f32,

    #[inspectable(ignore)]
    pub target: Entity,
}

impl SteeringBehavior for Interpose {}

impl Interpose {
    pub fn force(
        &self,
        steering: &Steering,
        physical: &PhysicalQueryItem,
        physicals: &Query<PhysicalQuery>,
    ) -> Vec2 {
        let target = physicals.get(self.target).unwrap();
        let target_position = target.transform.translation.truncate();

        let to_target = (target_position - steering.target).normalize_or_zero();
        arrive_force(
            steering.target + to_target * self.distance,
            physical,
            Deceleration::Normal,
        )
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct InterposeQueryMut<'w> {
    pub interpose: &'w Interpose,
    pub steering: &'w mut Steering,
}

#[derive(Debug, Default, Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct ObstacleAvoidance {
    pub box_length: f32,
}

#[derive(Debug, Default, Component)]
pub struct ObstacleAvoidanceDebug;
