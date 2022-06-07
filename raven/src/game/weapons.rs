use bevy_inspector_egui::prelude::*;
use strum_macros::EnumIter;

// TODO: pull ammo parameters from a config

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Inspectable, EnumIter)]
pub enum Ammo {
    Shell,
    Rocket,
    Slug,
}

impl Ammo {
    pub fn get_max_amount(&self) -> usize {
        match self {
            Self::Shell => 100,
            Self::Rocket => 100,
            Self::Slug => 100,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Inspectable, EnumIter)]
pub enum Weapon {
    Blaster,
    Shotgun,
    RocketLauncher,
    Railgun,
}

impl Weapon {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Blaster => "Blaster",
            Self::Shotgun => "Shotgun",
            Self::RocketLauncher => "Rocket Launcher",
            Self::Railgun => "Railgun",
        }
    }
}
