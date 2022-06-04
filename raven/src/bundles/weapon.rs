use bevy::prelude::*;

use crate::components::weapon::*;

#[derive(Debug, Default, Bundle)]
pub struct BlasterBundle {
    pub weapon: Weapon,
    pub blaster: Blaster,
}

#[derive(Debug, Default, Bundle)]
pub struct ShotgunBundle {
    pub weapon: Weapon,
    pub shotgun: Shotgun,
}

#[derive(Debug, Default, Bundle)]
pub struct RocketLauncherBundle {
    pub weapon: Weapon,
    pub rocket_launcher: RocketLauncher,
}

#[derive(Debug, Default, Bundle)]
pub struct RailgunBundle {
    pub weapon: Weapon,
    pub railgun: Railgun,
}
