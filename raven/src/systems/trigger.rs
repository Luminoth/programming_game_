use bevy::prelude::*;

use crate::components::bot::*;
use crate::components::collision::*;
use crate::components::inventory::*;
use crate::components::physics::*;
use crate::components::trigger::*;
use crate::game::PHYSICS_STEP;

pub fn update(time: Res<Time>, mut triggeres: Query<&mut Trigger>) {
    for mut trigger in triggeres.iter_mut() {
        trigger.update(time.delta_seconds());
    }
}

pub fn check_bot_collision(
    mut triggers: Query<(&mut Trigger, &Transform, &Bounds)>,
    mut bots: Query<(BotQueryMut, PhysicalQuery, &mut Inventory, &Bounds, &Name)>,
) {
    for (mut bot, bot_physical, mut inventory, bot_bounds, name) in bots.iter_mut() {
        let bot_position = bot_physical.transform.translation.truncate();
        let bot_future_position = bot_physical
            .physical
            .future_position(bot_physical.transform, PHYSICS_STEP);

        let v = bot_future_position - bot_position;
        let distance = v.length();
        let direction = v.normalize_or_zero();

        for (mut trigger, transform, bounds) in triggers.iter_mut() {
            let position = transform.translation.truncate();

            // TODO: need to account for bot bounds in raycast

            let contains = bot_bounds.contains(bot_position, position);

            if bounds
                .ray_intersects(position, bot_position, direction, distance)
                .is_some()
            {
                if !contains {
                    trigger.trigger(&mut bot.bot, &mut inventory, name);
                    break;
                }
            }
        }
    }
}
