pub mod cooldown;
pub mod weapons;

// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

// sprite sorting
pub const BOT_SORT: f32 = 2.0;
pub const CORPSE_SORT: f32 = 2.0;
pub const PROJECTILE_SORT: f32 = 2.0;
pub const WALL_SORT: f32 = 3.0;
pub const DEBUG_SORT: f32 = 100.0;

pub const BOT_RADIUS: f32 = 1.0;
pub const CORPSE_RADIUS: f32 = 1.0;
pub const BOLT_RADIUS: f32 = 0.2;
pub const PELLET_RADIUS: f32 = 0.1;
pub const ROCKET_RADIUS: f32 = 0.35;
pub const SLUG_RADIUS: f32 = 0.2;
