pub mod actor;
pub mod agent;
pub mod bot;
pub mod camera;
pub mod collision;
pub mod corpse;
pub mod inventory;
pub mod physics;
pub mod projectile;
pub mod spawnpoint;
pub mod steering;
pub mod trigger;
pub mod wall;
pub mod weapon;

use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Model;
