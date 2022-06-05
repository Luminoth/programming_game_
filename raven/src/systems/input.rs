use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::camera::*;
use crate::components::collision::*;
use crate::components::weapon::*;
use crate::util::*;

pub fn select_bot(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<CameraQuery, With<MainCamera>>,
    bots: Query<(Entity, &Bot, &Name, BoundsQuery<BoundingCircle>)>,
    selected: Query<Entity, With<SelectedBot>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let camera = camera.single();
        let window = windows.get_primary().unwrap();
        if let Some(mouse_position) = get_mouse_position((camera.camera, camera.transform), window)
        {
            for (entity, bot, name, bounds) in bots.iter() {
                if bounds.bounds.contains(bounds.transform, mouse_position) {
                    info!("selecting bot '{}'", name);
                    bot.select(&mut commands, entity, selected.optional_single());
                    break;
                }
            }
        }
    }
}

// TODO: this isn't great, we just want to process this once
// not once for every weapon type (a bot will only have a single weapon)
pub fn fire_weapon<T>(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<CameraQuery, With<MainCamera>>,
    mut weapons: Query<&mut T>,
    selected: Query<(Entity, &Transform, &Name), With<SelectedBot>>,
) where
    T: WeaponType,
{
    if buttons.just_released(MouseButton::Right) {
        if let Some((selected, transform, name)) = selected.optional_single() {
            let camera = camera.single();
            let window = windows.get_primary().unwrap();
            if let Some(mouse_position) =
                get_mouse_position((camera.camera, camera.transform), window)
            {
                if let Ok(mut weapon) = weapons.get_mut(selected) {
                    if !weapon.is_empty() {
                        let position = transform.translation.truncate();

                        info!(
                            "[{}]: firing weapon '{}' at {} from {}!",
                            name,
                            T::name(),
                            mouse_position,
                            position
                        );

                        weapon.fire(
                            &mut commands,
                            position,
                            (mouse_position - position).normalize_or_zero(),
                        );
                    } else {
                        warn!("[{}] weapon '{}' empty!", name, T::name());
                    }
                } else {
                    info!("[{}] not equipped with weapon '{}'", name, T::name());
                }
            }
        } else {
            info!("no bot selected for firing weapon '{}'", T::name());
        }
    }
}
