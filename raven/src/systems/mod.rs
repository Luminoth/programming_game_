pub mod debug;
pub mod physics;
pub mod projectile;

use bevy::prelude::*;

use crate::components::camera::*;
use crate::components::weapon::*;
use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Physics,

    Weapons,
    Projectiles,
}

pub fn test_fire<T>(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<(&Camera, &Transform), With<MainCamera>>,
    mut weapons: Query<(&Transform, &mut T)>,
) where
    T: Weapon,
{
    if buttons.just_released(MouseButton::Right) {
        let camera = camera.single();
        let window = windows.get_primary().unwrap();

        if let Some(mouse_position) = util::get_mouse_position(camera, window) {
            for (transform, mut weapon) in weapons.iter_mut() {
                let position = transform.translation.truncate();

                info!(
                    "firing {} at {} from {}!",
                    T::name(),
                    mouse_position,
                    position
                );

                weapon.fire(
                    &mut commands,
                    position,
                    (mouse_position - position).normalize_or_zero(),
                );
            }
        }
    }
}
