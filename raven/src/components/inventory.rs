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

impl Inventory {
    pub fn has_weapon(&self, weapon: Weapon) -> bool {
        *self.weapons.get(&weapon).unwrap()
    }

    pub fn get_ammo_amount(&self, weapon: Weapon) -> usize {
        if weapon == Weapon::Blaster {
            return 0;
        }

        let ammo = weapon.get_ammo();
        *self.ammo.get(&ammo).unwrap()
    }

    pub fn increase_ammo(&mut self, weapon: Weapon, amount: usize) {
        if weapon == Weapon::Blaster {
            return;
        }

        let ammo = weapon.get_ammo();
        let current_amount = *self.ammo.get(&ammo).unwrap();
        let available = ammo.get_max_amount() - current_amount;
        let amount = available.min(amount);

        if amount > 0 {
            self.ammo.insert(ammo, current_amount + amount);
        }
    }

    pub fn decrease_ammo(&mut self, weapon: Weapon, amount: usize) {
        if weapon == Weapon::Blaster {
            return;
        }

        let ammo = weapon.get_ammo();
        let current_amount = *self.ammo.get(&ammo).unwrap();
        let amount = current_amount.min(amount);

        if amount > 0 {
            self.ammo.insert(ammo, current_amount - amount);
        }
    }

    pub fn fill(&mut self, name: impl AsRef<str>) {
        info!("[{}]: filling inventory!", name.as_ref());

        for weapon in Weapon::iter() {
            self.weapons.insert(weapon, true);
        }

        for ammo_ in Ammo::iter() {
            self.ammo.insert(ammo_, ammo_.get_max_amount());
        }
    }
}
