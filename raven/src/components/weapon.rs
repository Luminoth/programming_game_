use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::projectile::*;
use crate::components::projectile::*;

// TODO: weapon cooldown

// TODO: pull these from a config
const BOLT_SPEED: f32 = 50.0;
const PELLET_SPEED: f32 = 25.0;
const ROCKET_SPEED: f32 = 10.0;
const SLUG_SPEED: f32 = 100.0;

pub trait Weapon {
    fn is_empty(&self) -> bool;

    fn fire(&mut self, commands: &mut Commands, transform: &Transform, direction: Vec2);
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Blaster;

impl Weapon for Blaster {
    fn is_empty(&self) -> bool {
        false
    }

    fn fire(&mut self, commands: &mut Commands, transform: &Transform, direction: Vec2) {
        let position = transform.translation.truncate();

        // TODO: offset
        ProjectileBundle::<Bolt>::spawn_at_position(commands, position, direction * BOLT_SPEED);
        ProjectileBundle::<Bolt>::spawn_at_position(commands, position, direction * BOLT_SPEED);
        ProjectileBundle::<Bolt>::spawn_at_position(commands, position, direction * BOLT_SPEED);
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Shotgun;

impl Weapon for Shotgun {
    fn is_empty(&self) -> bool {
        false
    }

    fn fire(&mut self, commands: &mut Commands, transform: &Transform, direction: Vec2) {
        if self.is_empty() {
            return;
        }

        let position = transform.translation.truncate();

        // TODO: spread
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
        ProjectileBundle::<Pellet>::spawn_at_position(commands, position, direction * PELLET_SPEED);
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct RocketLauncher;

impl Weapon for RocketLauncher {
    fn is_empty(&self) -> bool {
        false
    }

    fn fire(&mut self, commands: &mut Commands, transform: &Transform, direction: Vec2) {
        if self.is_empty() {
            return;
        }

        let position = transform.translation.truncate();

        ProjectileBundle::<Rocket>::spawn_at_position(commands, position, direction * ROCKET_SPEED);
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Railgun;

impl Weapon for Railgun {
    fn is_empty(&self) -> bool {
        false
    }

    fn fire(&mut self, commands: &mut Commands, transform: &Transform, direction: Vec2) {
        if self.is_empty() {
            return;
        }

        let position = transform.translation.truncate();

        ProjectileBundle::<Slug>::spawn_at_position(commands, position, direction * SLUG_SPEED);
    }
}
