use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::inventory::*;
use crate::components::physics::*;
use crate::components::trigger::*;

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut triggeres: Query<(Entity, &mut Trigger)>,
) {
    for (entity, mut trigger) in triggeres.iter_mut() {
        trigger.update(&mut commands, entity, time.delta_seconds());
    }
}

pub fn check_bot_collision(
    mut triggers: Query<(&mut Trigger, &Transform, &Bounds)>,
    mut bots: Query<(
        Entity,
        BotQueryMut,
        PhysicalQuery,
        &mut Inventory,
        &Bounds,
        &Name,
    )>,
) {
    for (mut trigger, transform, bounds) in triggers.iter_mut() {
        let position = transform.translation.truncate();

        for (entity, mut bot, bot_physical, mut inventory, bot_bounds, name) in bots.iter_mut() {
            // TODO: need to account for bot bounds in raycast

            let contains = bot_bounds.contains(bot_physical.physical.cache.position, position);
            if contains {
                // don't re-trigger
                continue;
            }

            if bounds
                .ray_intersects(
                    position,
                    bot_physical.physical.cache.position,
                    bot_physical.physical.cache.heading,
                    bot_physical.physical.cache.max_distance,
                )
                .is_some()
            {
                trigger.trigger(entity, &mut bot.bot, &mut inventory, name);
                break;
            }
        }
    }
}
