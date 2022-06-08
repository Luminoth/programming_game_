use bevy::prelude::*;

use crate::components::weapon::*;

pub fn update(time: Res<Time>, mut weapons: Query<&mut EquippedWeapon>) {
    for mut weapon in weapons.iter_mut() {
        weapon.cooldown.tick(time.delta_seconds());
    }
}
