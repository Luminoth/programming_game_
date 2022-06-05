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

pub trait WeaponType: Default + Component {
    fn name() -> &'static str;

    fn is_empty(&self) -> bool;

    fn fire(&mut self, commands: &mut Commands, position: Vec2, direction: Vec2);
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Blaster;

impl WeaponType for Blaster {
    fn name() -> &'static str {
        "Blaster"
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn fire(&mut self, commands: &mut Commands, position: Vec2, direction: Vec2) {
        // TODO: offset
        ProjectileBundle::<Bolt>::spawn_at_position(commands, position, direction * BOLT_SPEED);
        ProjectileBundle::<Bolt>::spawn_at_position(commands, position, direction * BOLT_SPEED);
        ProjectileBundle::<Bolt>::spawn_at_position(commands, position, direction * BOLT_SPEED);
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Shotgun {
    ammo: usize,
}

impl WeaponType for Shotgun {
    fn name() -> &'static str {
        "Shotgun"
    }

    fn is_empty(&self) -> bool {
        self.ammo < 1
    }

    fn fire(&mut self, commands: &mut Commands, position: Vec2, direction: Vec2) {
        if self.is_empty() {
            return;
        }

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

        self.ammo -= 1;
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct RocketLauncher {
    ammo: usize,
}

impl WeaponType for RocketLauncher {
    fn name() -> &'static str {
        "RocketLauncher"
    }

    fn is_empty(&self) -> bool {
        self.ammo < 1
    }

    fn fire(&mut self, commands: &mut Commands, position: Vec2, direction: Vec2) {
        if self.is_empty() {
            return;
        }

        ProjectileBundle::<Rocket>::spawn_at_position(commands, position, direction * ROCKET_SPEED);

        self.ammo -= 1;
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct Railgun {
    ammo: usize,
}

impl WeaponType for Railgun {
    fn name() -> &'static str {
        "Railgun"
    }

    fn is_empty(&self) -> bool {
        self.ammo < 1
    }

    fn fire(&mut self, commands: &mut Commands, position: Vec2, direction: Vec2) {
        if self.is_empty() {
            return;
        }

        ProjectileBundle::<Slug>::spawn_at_position(commands, position, direction * SLUG_SPEED);

        self.ammo -= 1;
    }
}
