#![allow(non_snake_case)]

use bevy::prelude::*;

use crate::components::team::*;
use crate::game::team::*;

pub fn GlobalState_on_message<T>(
    mut commands: Commands,
    mut message_events: EventReader<GoalKeeperDispatchedMessageEvent>,
    mut goal_keeper: Query<(Entity, GoalKeeperQueryMut<T>)>,
) where
    T: TeamColorMarker,
{
    let (entity, mut goal_keeper) = goal_keeper.single_mut();

    for event in message_events.iter() {
        if entity != event.receiver.unwrap() {
            continue;
        }

        match event.message {
            GoalKeeperMessage::GoHome => {
                goal_keeper.player.home_region = goal_keeper.player.default_region;

                goal_keeper.state_machine.change_state(
                    &mut commands,
                    entity,
                    GoalKeeperState::ReturnHome,
                );
            }
            GoalKeeperMessage::ReceiveBall => {
                goal_keeper.state_machine.change_state(
                    &mut commands,
                    entity,
                    GoalKeeperState::InterceptBall,
                );
            }
        }
    }
}
