use bevy_inspector_egui::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, Eq, PartialEq, Hash, Inspectable, EnumIter)]
pub enum Ammo {
    Shell,
    Rocket,
    Slug,
}

#[derive(Debug, Eq, PartialEq, Hash, Inspectable, EnumIter)]
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
