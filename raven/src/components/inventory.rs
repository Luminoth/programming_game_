use std::collections::HashMap;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use strum::IntoEnumIterator;

use crate::game::weapons::*;

#[derive(Debug, Component, Inspectable)]
pub struct Inventory {
    #[inspectable(ignore)]
    pub weapons: HashMap<Weapon, bool>,

    #[inspectable(ignore)]
    pub ammo: HashMap<Ammo, usize>,
}

impl Default for Inventory {
    fn default() -> Self {
        let mut weapons = HashMap::new();
        for weapon in Weapon::iter() {
            weapons.insert(weapon, false);
        }
        weapons.insert(Weapon::Blaster, true);

        let mut ammo = HashMap::new();
        for ammo_ in Ammo::iter() {
            ammo.insert(ammo_, 0);
        }

        Self { weapons, ammo }
    }
}
