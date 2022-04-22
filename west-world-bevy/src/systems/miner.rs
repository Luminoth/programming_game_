#![allow(non_snake_case)]

use bevy::prelude::*;
use chrono::prelude::*;

use crate::components::miner::*;
use crate::events::messaging::MessageEvent;
use crate::game::miner::*;
use crate::game::Location;
use crate::resources::messaging::MessageDispatcher;

pub fn update(mut query: Query<&mut Stats>) {
    for mut stats in query.iter_mut() {
        stats.update();
    }
}

pub fn EnterMineAndDigForNugget_enter(
    mut query: Query<MinerQuery, With<MinerStateEnterMineAndDigForNuggetEnter>>,
) {
    for mut miner in query.iter_mut() {
        if miner.miner.location != Location::GoldMine {
            info!("{}: Walkin' to the gold mine", miner.name.as_ref());

            miner.miner.location = Location::GoldMine;
        }
    }
}

pub fn EnterMineAndDigForNugget_execute(
    mut commands: Commands,
    mut query: Query<(Entity, MinerQuery), With<MinerStateEnterMineAndDigForNuggetExecute>>,
) {
    for (entity, mut miner) in query.iter_mut() {
        miner.stats.mine_gold();

        info!("{}: Pickin' up a nugget", miner.name.as_ref());

        if miner.stats.are_pockets_full() {
            miner.state_machine.change_state(
                &mut commands,
                entity,
                MinerState::VisitBankAndDepositGold,
            );
        } else if miner.stats.is_thirsty() {
            miner
                .state_machine
                .change_state(&mut commands, entity, MinerState::QuenchThirst);
        }
    }
}

pub fn EnterMineAndDigForNugget_exit(
    query: Query<MinerQuery, With<MinerStateEnterMineAndDigForNuggetExit>>,
) {
    for miner in query.iter() {
        info!(
            "{}: Ah'm leavin' the gold mine with mah pockets full o' sweet gold",
            miner.name.as_ref()
        )
    }
}

pub fn VisitBankAndDepositGold_enter(
    mut query: Query<MinerQuery, With<MinerStateVisitBankAndDepositGoldEnter>>,
) {
    for mut miner in query.iter_mut() {
        if miner.miner.location != Location::Bank {
            info!("{}: Goin' to the bank. Yes siree", miner.name.as_ref());

            miner.miner.location = Location::Bank;
        }
    }
}

pub fn VisitBankAndDepositGold_execute(
    mut commands: Commands,
    mut query: Query<(Entity, MinerQuery), With<MinerStateVisitBankAndDepositGoldExecute>>,
) {
    for (entity, mut miner) in query.iter_mut() {
        miner.stats.transfer_gold_to_wealth();

        info!(
            "{}: Depositing gold. Total savings now: {}",
            miner.name.as_ref(),
            miner.stats.wealth()
        );

        if miner.stats.wealth() >= COMFORT_LEVEL {
            info!(
                "{}: WooHoo! Rich enough for now. Back home to mah li'lle lady",
                miner.name.as_ref()
            );

            miner.state_machine.change_state(
                &mut commands,
                entity,
                MinerState::GoHomeAndSleepTilRested,
            );
        } else {
            miner.state_machine.change_state(
                &mut commands,
                entity,
                MinerState::EnterMineAndDigForNugget,
            );
        }
    }
}

pub fn VisitBankAndDepositGold_exit(
    query: Query<MinerQuery, With<MinerStateVisitBankAndDepositGoldExit>>,
) {
    for miner in query.iter() {
        info!("{}: Leavin' the bank", miner.name.as_ref());
    }
}

pub fn GoHomeAndSleepTilRested_enter(
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut query: Query<
        (Entity, MinerQuery, &MinerWife),
        With<MinerStateGoHomeAndSleepTilRestedEnter>,
    >,
) {
    for (entity, mut miner, wife) in query.iter_mut() {
        if miner.miner.location != Location::Shack {
            info!("{}: Walkin' home", miner.name.as_ref());

            miner.miner.location = Location::Shack;

            message_dispatcher.dispatch_message(wife.wife_id, MessageEvent::HiHoneyImHome(entity));
        }
    }
}

pub fn GoHomeAndSleepTilRested_execute(
    mut commands: Commands,
    mut query: Query<(Entity, MinerQuery), With<MinerStateGoHomeAndSleepTilRestedExecute>>,
) {
    for (entity, mut miner) in query.iter_mut() {
        if !miner.stats.is_fatigued() {
            info!(
                "{}: What a God darn fantastic nap! Time to find more gold",
                miner.name.as_ref()
            );

            miner.state_machine.change_state(
                &mut commands,
                entity,
                MinerState::EnterMineAndDigForNugget,
            );
        } else {
            miner.stats.rest();

            info!("{}: ZZZZ... ", miner.name.as_ref());
        }
    }
}

pub fn GoHomeAndSleepTilRested_exit(
    query: Query<MinerQuery, With<MinerStateGoHomeAndSleepTilRestedExit>>,
) {
    for miner in query.iter() {
        info!("{}: Leaving the house", miner.name.as_ref());
    }
}

pub fn GoHomeAndSleepTilRested_on_message(
    mut commands: Commands,
    mut message_events: EventReader<(Entity, MessageEvent)>,
    mut query: Query<(Entity, MinerQuery), With<MinerStateGoHomeAndSleepTilRestedExecute>>,
) {
    for (receiver, event) in message_events.iter() {
        if let Ok((entity, mut miner)) = query.get_mut(*receiver) {
            match event {
                MessageEvent::StewIsReady(_) => {
                    let now = Utc::now();

                    debug!(
                        "Message handled by {} at time: {}",
                        miner.name.as_ref(),
                        now
                    );
                    info!("{}: Ok hun, ahm a-comin'!", miner.name.as_ref());

                    miner
                        .state_machine
                        .change_state(&mut commands, entity, MinerState::EatStew);
                }
                _ => (),
            }
        }
    }
}

pub fn QuenchThirst_enter(mut query: Query<MinerQuery, With<MinerStateQuenchThirstEnter>>) {
    for mut miner in query.iter_mut() {
        if miner.miner.location != Location::Saloon {
            info!(
                "{}: Boy, ah sure is thusty! Walking to the saloon",
                miner.name.as_ref()
            );

            miner.miner.location = Location::Shack;
        }
    }
}

pub fn QuenchThirst_execute(
    mut commands: Commands,
    mut query: Query<(Entity, MinerQuery), With<MinerStateQuenchThirstExecute>>,
) {
    for (entity, mut miner) in query.iter_mut() {
        if miner.stats.is_thirsty() {
            miner.stats.buy_and_drink_a_whiskey();

            info!("{}: That's mighty fine sippin liquer", miner.name.as_ref());

            miner.state_machine.change_state(
                &mut commands,
                entity,
                MinerState::EnterMineAndDigForNugget,
            );
        } else {
            unreachable!();
        }
    }
}

pub fn QuenchThirst_exit(query: Query<MinerQuery, With<MinerStateQuenchThirstExit>>) {
    for miner in query.iter() {
        info!("{}: Leaving the saloon, feelin' good", miner.name.as_ref());
    }
}

pub fn EatStew_enter(query: Query<MinerQuery, With<MinerStateEatStewEnter>>) {
    for miner in query.iter() {
        info!("{}: Smells reaaal good Elsa!", miner.name.as_ref());
    }
}

pub fn EatStew_execute(
    mut commands: Commands,
    mut query: Query<(Entity, MinerQuery), With<MinerStateEatStewExecute>>,
) {
    for (entity, mut miner) in query.iter_mut() {
        info!("{}: Tastes reaaal good too!", miner.name.as_ref());

        miner
            .state_machine
            .revert_to_previous_state(&mut commands, entity);
    }
}

pub fn EatStew_exit(query: Query<MinerQuery, With<MinerStateEatStewExit>>) {
    for miner in query.iter() {
        info!(
            "{}: Thankya li'lle lady. Ah better get back to whatever ah wuz doin'",
            miner.name.as_ref()
        );
    }
}
