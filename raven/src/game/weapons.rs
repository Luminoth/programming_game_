use bevy_inspector_egui::prelude::*;
use strum_macros::EnumIter;

// TODO: pull ammo parameters from a config

// TODO: pull weapon parameters from a config

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Inspectable, EnumIter)]
pub enum Ammo {
    Shell,
    Rocket,
    Slug,
}

impl Ammo {
    pub fn get_max_amount(&self) -> usize {
        match self {
            Self::Shell => 10,
            Self::Rocket => 5,
            Self::Slug => 5,
        }
    }

    pub fn get_trigger_amount(&self) -> usize {
        match self {
            Self::Shell => 5,
            Self::Rocket => 1,
            Self::Slug => 1,
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

    pub fn get_ammo(&self) -> Ammo {
        match self {
            Self::Blaster => panic!("Blasters don't use ammo!"),
            Self::Shotgun => Ammo::Shell,
            Self::RocketLauncher => Ammo::Rocket,
            Self::Railgun => Ammo::Slug,
        }
    }

    pub fn get_cooldown_seconds(&self) -> f32 {
        match self {
            Self::Blaster => 0.33,
            Self::Shotgun => 1.0,
            Self::RocketLauncher => 0.66,
            Self::Railgun => 1.0,
        }
    }
}
