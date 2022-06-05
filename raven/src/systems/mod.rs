pub mod debug;
pub mod physics;
pub mod projectile;

use bevy::prelude::*;

use crate::components::agent::*;
use crate::components::camera::*;
use crate::components::collision::*;
use crate::components::weapon::*;
use crate::util::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Physics,
    Input,

    Weapons,
    Projectiles,
}

pub fn test_select(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<(&Camera, &Transform), With<MainCamera>>,
    agents: Query<(Entity, &Agent, &Transform, &BoundingCircle, &Name), With<Agent>>,
    selected_agent: Query<Entity, With<SelectedAgent>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let camera = camera.single();
        let window = windows.get_primary().unwrap();

        if let Some(mouse_position) = get_mouse_position(camera, window) {
            for (entity, agent, transform, bounds, name) in agents.iter() {
                if bounds.contains(transform, mouse_position) {
                    info!("selecting '{}'", name);
                    agent.select(&mut commands, entity, selected_agent.optional_single());
                }
            }
        }
    }
}

pub fn test_fire<T>(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    camera: Query<(&Camera, &Transform), With<MainCamera>>,
    mut weapons: Query<(&Transform, &mut T)>,
) where
    T: WeaponType,
{
    if buttons.just_released(MouseButton::Right) {
        let camera = camera.single();
        let window = windows.get_primary().unwrap();

        if let Some(mouse_position) = get_mouse_position(camera, window) {
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
