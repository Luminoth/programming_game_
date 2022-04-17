use std::cmp::Ordering;
use std::collections::BinaryHeap;

use bevy::prelude::*;
use chrono::prelude::*;

use crate::events::messaging::MessageEvent;

// TODO: this should live in the events module
// also the name isn't very good
pub struct DispatchedMessageEvent {
    pub receiver: Option<Entity>,
    pub message: MessageEvent,
}

#[derive(Debug)]
struct Telegram {
    dispatch_time: i64,

    pub receiver: Option<Entity>,

    pub message: MessageEvent,
}

impl Eq for Telegram {}

impl PartialEq for Telegram {
    fn eq(&self, other: &Self) -> bool {
        self.dispatch_time == other.dispatch_time
            && self.receiver == other.receiver
            && self.message == other.message
    }
}

impl PartialOrd for Telegram {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Telegram {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: this may need to be reversed
        // so that we get a time-ordered min-heap?
        self.dispatch_time.cmp(&other.dispatch_time)
    }
}

impl Telegram {
    fn new(dispatch_time: i64, receiver: Option<Entity>, message: MessageEvent) -> Self {
        Self {
            dispatch_time,
            receiver,
            message,
        }
    }
}

#[derive(Default)]
pub struct MessageDispatcher {
    queue: BinaryHeap<Telegram>,
}

impl MessageDispatcher {
    pub fn dispatch_deferred_messages(
        &mut self,
        message_events: &mut EventWriter<DispatchedMessageEvent>,
    ) {
        let now = Utc::now().timestamp_millis();

        loop {
            if let Some(telegram) = self.queue.peek() {
                if telegram.dispatch_time > now {
                    return;
                }
            } else {
                return;
            }

            let telegram = self.queue.pop().unwrap();
            self.discharge(telegram, message_events);
        }
    }

    fn discharge(
        &self,
        telegram: Telegram,
        message_events: &mut EventWriter<DispatchedMessageEvent>,
    ) {
        message_events.send(DispatchedMessageEvent {
            receiver: telegram.receiver,
            message: telegram.message,
        });
    }

    pub fn dispatch_message(&mut self, receiver: Option<Entity>, message: MessageEvent) {
        // we always defer so that entities sending messages (as events)
        // in response to events can work
        self.defer_dispatch_message(receiver, message, 0.0);
    }

    pub fn defer_dispatch_message(
        &mut self,
        receiver: Option<Entity>,
        message: MessageEvent,
        delay_seconds: f64,
    ) {
        let now = Utc::now().timestamp_millis();
        let telegram = Telegram::new(now + (delay_seconds * 1000.0) as i64, receiver, message);

        self.queue.push(telegram);
    }
}
