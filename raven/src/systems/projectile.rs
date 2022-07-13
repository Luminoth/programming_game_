use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::inventory::*;
use crate::components::physics::*;
use crate::components::projectile::*;
use crate::components::wall::*;
use crate::ORTHO_SIZE;

pub fn check_bounds(
    mut commands: Commands,
    windows: Res<Windows>,
    projectiles: Query<(Entity, PhysicalQuery, &Bounds, &Name), With<Projectile>>,
) {
    let window = windows.get_primary().unwrap();
    let aspect_ratio = window.width() / window.height();

    let max_x = ORTHO_SIZE;
    let max_y = ORTHO_SIZE / aspect_ratio;

    for (entity, physical, bounds, name) in projectiles.iter() {
        let projectile_min_x =
            physical.physical.cache.future_position.x + bounds.center().x - bounds.width();
        let projectile_min_y =
            physical.physical.cache.future_position.y + bounds.center().y - bounds.height();
        let projectile_max_x =
            physical.physical.cache.future_position.x + bounds.center().x + bounds.width();
        let projectile_max_y =
            physical.physical.cache.future_position.y + bounds.center().y + bounds.height();

        if projectile_min_x < -max_x
            || projectile_max_x > max_x
            || projectile_min_y < -max_y
            || projectile_max_y > max_y
        {
            info!("projectile '{}' is out of bounds", name);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn check_wall_collision(
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile, PhysicalQuery, &Bounds, &Name)>,
    walls: Query<WallQuery>,
    mut bots: Query<(Entity, BotQueryMut, &mut Inventory, PhysicalQuery, &Bounds)>,
) {
    for (entity, projectile, physical, bounds, name) in projectiles.iter() {
        // TODO: need to account for bounds width / height

        let projectile_position = physical.physical.cache.position + bounds.center();
        let heading = physical.physical.cache.heading * physical.physical.cache.max_distance;

        for wall in walls.iter() {
            let wall_position = wall.transform.translation.truncate();
            let wall_from = wall.wall.from(wall_position);
            let wall_to = wall.wall.to(wall_position);

            /*let contains = wall_bounds.contains(wall_position, projectile_position);
            if contains {
                // TODO: push back out of the wall?
                continue;
            }*/

            if let Some((_, hit)) =
                line_intersection(wall_from, wall_to, projectile_position, heading)
            {
                info!("projectile '{}' hit a wall at {}", name, hit);
                projectile.on_impact(&mut commands, entity, hit, bots.iter_mut());

                commands.entity(entity).despawn_recursive();
                break;
            }
        }
    }
}

// TODO: bots move so we really need to do a ray vs ray intersection?
pub fn check_bot_collision(
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile, PhysicalQuery, &Bounds, &Name)>,
    mut bots: Query<(Entity, BotQueryMut, &mut Inventory, PhysicalQuery, &Bounds)>,
) {
    for (entity, projectile, physical, bounds, name) in projectiles.iter() {
        // TODO: need to account for bounds width / height

        let projectile_position = physical.physical.cache.position + bounds.center();

        for (bot_entity, mut bot, mut inventory, bot_physical, bot_bounds) in bots.iter_mut() {
            if bot_entity == projectile.get_owner() {
                continue;
            }

            let bot_position = bot_physical.physical.cache.position + bot_bounds.center();

            let contains = bot_bounds.contains(bot_position, projectile_position);
            if contains {
                // don't re-collide
                continue;
            }

            if let Some(hit) = bot_bounds.ray_intersects(
                bot_position,
                projectile_position,
                physical.physical.cache.heading,
                physical.physical.cache.max_distance,
            ) {
                info!("projectile '{}' hit bot '{}' at {}!", name, bot.name, hit);
                bot.bot.damage(
                    &mut commands,
                    bot_entity,
                    bot_physical.transform,
                    &mut inventory,
                    bot.name,
                    projectile.get_damage(),
                );

                projectile.on_impact(&mut commands, entity, hit, bots.iter_mut());

                if !matches!(projectile, Projectile::Slug(_)) {
                    commands.entity(entity).despawn_recursive();
                }
                break;
            }
        }
    }
}
