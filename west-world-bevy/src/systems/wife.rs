#![allow(non_snake_case)]

use bevy::prelude::*;
use chrono::prelude::*;
use rand::Rng;

use crate::components::wife::*;
use crate::events::messaging::MessageEvent;
use crate::game::wife::*;
use crate::resources::messaging::MessageDispatcher;

pub fn GlobalState_execute(mut commands: Commands, mut query: Query<(Entity, WifeQuery)>) {
    let mut rng = rand::thread_rng();

    for (entity, mut wife) in query.iter_mut() {
        debug!("executing wife global state for {}", wife.name.as_ref());

        if rng.gen::<f32>() < BATHROOM_CHANCE {
            wife.state_machine
                .change_state(&mut commands, entity, WifeState::VisitBathroom);
        }
    }
}

pub fn GlobalState_on_message(
    mut commands: Commands,
    mut message_events: EventReader<(Entity, MessageEvent)>,
    mut query: Query<(Entity, WifeQuery)>,
) {
    for (receiver, event) in message_events.iter() {
        if let Ok((entity, mut wife)) = query.get_mut(*receiver) {
            match event {
                MessageEvent::HiHoneyImHome(_) => {
                    let now = Utc::now();

                    debug!("Message handled by {} at time: {}", wife.name.as_ref(), now);
                    info!(
                        "{}: Hi honey. Let me make you some of mah fine country stew",
                        wife.name.as_ref()
                    );

                    wife.state_machine
                        .change_state(&mut commands, entity, WifeState::CookStew);
                }
                _ => (),
            }
        }
    }
}

pub fn DoHouseWork_execute(query: Query<WifeQuery, With<WifeStateDoHouseWorkExecute>>) {
    let mut rng = rand::thread_rng();

    for wife in query.iter() {
        match rng.gen_range(0..=2) {
            0 => info!("{}: Moppin' the floor", wife.name.as_ref()),
            1 => info!("{}: Washin' the dishes", wife.name.as_ref()),
            2 => info!("{}: Makin' the bed", wife.name.as_ref()),
            _ => unreachable!(),
        }
    }
}

pub fn VisitBathroom_enter(query: Query<WifeQuery, With<WifeStateVisitBathroomEnter>>) {
    for wife in query.iter() {
        info!(
            "{}: Walkin' to the can. Need to powda mah pretty li'lle nose",
            wife.name.as_ref()
        );
    }
}

pub fn VisitBathroom_execute(
    mut commands: Commands,
    mut query: Query<(Entity, WifeQuery), With<WifeStateVisitBathroomExecute>>,
) {
    for (entity, mut wife) in query.iter_mut() {
        info!("{}: Ahhhhhh! Sweet relief!", wife.name.as_ref());

        wife.state_machine
            .revert_to_previous_state(&mut commands, entity);
    }
}

pub fn VisitBathroom_exit(query: Query<WifeQuery, With<WifeStateVisitBathroomExit>>) {
    for wife in query.iter() {
        info!("{}: Leavin' the Jon", wife.name.as_ref());
    }
}

pub fn CookStew_enter(
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<(Entity, WifeQuery), With<WifeStateCookStewEnter>>,
) {
    for (entity, mut wife) in query.iter_mut() {
        if wife.wife.cooking {
            continue;
        }

        info!("{}: Puttin' the stew in the oven", wife.name.as_ref());

        message_dispatcher.defer_dispatch_message(entity, MessageEvent::StewIsReady(entity), 1.5);

        wife.wife.cooking = true;
    }
}

pub fn CookStew_on_message(
    mut commands: Commands,
    mut message_events: EventReader<(Entity, MessageEvent)>,
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<(Entity, WifeQuery, Option<&WifeMiner>), With<WifeStateCookStewExecute>>,
) {
    for (receiver, event) in message_events.iter() {
        if let Ok((entity, mut wife, miner)) = query.get_mut(*receiver) {
            match event {
                MessageEvent::StewIsReady(_) => {
                    let now = Utc::now();

                    debug!(
                        "Message received by {} at time: {}",
                        wife.name.as_ref(),
                        now
                    );
                    info!("{}: Stew ready! Let's eat", wife.name.as_ref());

                    message_dispatcher.dispatch_message(
                        miner.unwrap().miner_id,
                        MessageEvent::StewIsReady(entity),
                    );

                    wife.wife.cooking = false;

                    wife.state_machine
                        .change_state(&mut commands, entity, WifeState::DoHouseWork);
                }
                _ => (),
            }
        }
    }
}
