pub mod debug;
pub mod input;
pub mod physics;
pub mod projectile;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Physics,
    BoundsCheck,
    Input,
}
