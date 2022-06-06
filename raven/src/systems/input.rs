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
    possessed: Query<Entity, With<PossessedBot>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let camera = camera.single();
        let window = windows.get_primary().unwrap();
        if let Some(mouse_position) = get_mouse_position((camera.camera, camera.transform), window)
        {
            for (entity, bot, name, bounds) in bots.iter() {
                if bounds.bounds.contains(bounds.transform, mouse_position) {
                    bot.select(
                        &mut commands,
                        entity,
                        name.as_str(),
                        selected.optional_single(),
                        possessed.optional_single(),
                    );
                    break;
                }
            }
        }
    }
}

pub fn fire_weapon(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<CameraQuery, With<MainCamera>>,
    mut weapons: Query<&mut Weapon>,
    possessed: Query<(Entity, &Bot, &Transform, &Name), With<PossessedBot>>,
) {
    if buttons.just_released(MouseButton::Right) {
        if let Some((entity, bot, transform, name)) = possessed.optional_single() {
            let camera = camera.single();
            let window = windows.get_primary().unwrap();
            if let Some(mouse_position) =
                get_mouse_position((camera.camera, camera.transform), window)
            {
                let mut weapon = weapons.get_mut(entity).unwrap();
                bot.fire_weapon(
                    &mut commands,
                    &mut *weapon,
                    mouse_position,
                    transform,
                    name.as_str(),
                );
            }
        } else {
            info!("no bot possessed for firing weapon");
        }
    }
}

pub fn damage_bot(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut possessed: Query<(Entity, &mut Bot, &Transform, &Name), With<PossessedBot>>,
) {
    if keys.just_pressed(KeyCode::D) {
        if let Some((entity, mut bot, transform, name)) = possessed.optional_single_mut() {
            bot.damage(&mut commands, entity, transform, name.as_str(), 1);
        } else {
            info!("no bot possessed for damage");
        }
    }
}

pub fn kill_bot(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut possessed: Query<(Entity, &mut Bot, &Transform, &Name), With<PossessedBot>>,
) {
    if keys.just_pressed(KeyCode::K) {
        if let Some((entity, mut bot, transform, name)) = possessed.optional_single_mut() {
            bot.kill(&mut commands, entity, transform, name.as_str());
        } else {
            info!("no bot possessed for kill");
        }
    }
}
