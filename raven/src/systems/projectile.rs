use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::projectile::*;
use crate::components::wall::*;
use crate::ORTHO_SIZE;

pub fn check_bounds(
    mut commands: Commands,
    windows: Res<Windows>,
    projectiles: Query<(Entity, &Transform, &Name), With<Projectile>>,
) {
    let window = windows.get_primary().unwrap();
    let aspect_ratio = window.width() / window.height();

    for (entity, transform, name) in projectiles.iter() {
        let max_x = ORTHO_SIZE;
        let max_y = ORTHO_SIZE / aspect_ratio;

        let position = transform.translation.truncate();
        if position.x < -max_x || position.x > max_x || position.y < -max_y || position.y > max_y {
            info!("projectile '{}' is out of bounds", name);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn check_collision(
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile, &Transform, &Bounds, &Name)>,
    walls: Query<(&Transform, &Bounds), With<Wall>>,
    mut bots: Query<(Entity, &mut Bot, &Transform, &Bounds, &Name)>,
) {
    for (entity, projectile, transform, bounds, name) in projectiles.iter() {
        let mut despawned = false;
        for (wall_transform, wall_bounds) in walls.iter() {
            if bounds.bounds_intersects(transform, wall_bounds, wall_transform) {
                info!("projectile '{}' hit a wall", name);
                commands.entity(entity).despawn_recursive();

                despawned = true;
                break;
            }
        }

        if despawned {
            continue;
        }

        for (bot_entity, mut bot, bot_transform, bot_bounds, bot_name) in bots.iter_mut() {
            if bot_entity == projectile.get_owner() {
                continue;
            }

            if bounds.bounds_intersects(transform, bot_bounds, bot_transform) {
                info!("projectile '{}' hit bot '{}'!", name, bot_name);
                bot.damage(
                    &mut commands,
                    bot_entity,
                    bot_transform,
                    bot_name,
                    projectile.get_damage(),
                );
                commands.entity(entity).despawn_recursive();

                despawned = true;
                break;
            }
        }

        if despawned {
            continue;
        }
    }
}
