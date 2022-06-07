use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::camera::*;
use crate::components::collision::*;
use crate::components::inventory::*;
use crate::components::weapon::*;
use crate::game::weapons::*;
use crate::util::*;

pub fn select_bot(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<CameraQuery, With<MainCamera>>,
    bots: Query<(Entity, &Bot, &Name, BoundsQuery)>,
    selected: Query<(Entity, &Bot, &Name), With<SelectedBot>>,
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

pub fn deselect_bot(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    selected: Query<(Entity, &Bot, &Name), With<SelectedBot>>,
    possessed: Query<(Entity, &Bot, &Name), With<PossessedBot>>,
) {
    if keys.just_pressed(KeyCode::X) {
        if let Some(possessed) = possessed.optional_single() {
            possessed
                .1
                .deselect(&mut commands, possessed.0, possessed.2);
        } else if let Some(selected) = selected.optional_single() {
            selected.1.deselect(&mut commands, selected.0, selected.2);
        }
    }
}

pub fn select_weapon(
    keys: Res<Input<KeyCode>>,
    mut possessed: Query<(&mut EquippedWeapon, &Inventory, &Name), With<PossessedBot>>,
) {
    let mut weapon = None;
    if keys.just_pressed(KeyCode::Key1) {
        weapon = Some(Weapon::Blaster);
    } else if keys.just_pressed(KeyCode::Key2) {
        weapon = Some(Weapon::Shotgun);
    } else if keys.just_pressed(KeyCode::Key3) {
        weapon = Some(Weapon::RocketLauncher);
    } else if keys.just_pressed(KeyCode::Key4) {
        weapon = Some(Weapon::Railgun);
    }

    if let Some(weapon) = weapon {
        if let Some((mut equipped_weapon, inventory, name)) = possessed.optional_single_mut() {
            equipped_weapon.select(inventory, weapon, name.as_str());
        } else {
            info!("no bot possessed for select weapon '{}'", weapon.get_name());
        }
    }
}

pub fn fire_weapon(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<CameraQuery, With<MainCamera>>,
    mut possessed: Query<
        (&Bot, &mut EquippedWeapon, &mut Inventory, &Transform, &Name),
        With<PossessedBot>,
    >,
) {
    if buttons.just_released(MouseButton::Right) {
        if let Some((bot, mut weapon, mut inventory, transform, name)) =
            possessed.optional_single_mut()
        {
            let camera = camera.single();
            let window = windows.get_primary().unwrap();
            if let Some(mouse_position) =
                get_mouse_position((camera.camera, camera.transform), window)
            {
                bot.fire_weapon(
                    &mut commands,
                    &mut weapon,
                    &mut inventory,
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

pub fn fill_inventory(
    keys: Res<Input<KeyCode>>,
    mut possessed: Query<(&mut Inventory, &Name), With<PossessedBot>>,
) {
    if keys.just_pressed(KeyCode::F) {
        if let Some((mut inventory, name)) = possessed.optional_single_mut() {
            inventory.fill(name.as_str());
        } else {
            info!("no bot possessed for inventory fill");
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
