use std::cmp::Ordering;
use std::collections::BinaryHeap;

use bevy::prelude::*;
use chrono::prelude::*;

use crate::events::messaging::MessageEvent;
use crate::game::messaging::Message;

#[derive(Debug)]
struct Telegram {
    dispatch_time: i64,

    pub sender: Entity,
    pub receiver: Entity,

    pub message: Message,
}

impl Eq for Telegram {}

impl PartialEq for Telegram {
    fn eq(&self, other: &Self) -> bool {
        self.dispatch_time == other.dispatch_time
            && self.sender == other.sender
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
    fn new(dispatch_time: i64, sender: Entity, receiver: Entity, message: Message) -> Self {
        Self {
            dispatch_time,
            sender,
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
    pub fn dispatch_deferred_messages(&mut self, message_events: &mut EventWriter<MessageEvent>) {
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

    fn discharge(&self, telegram: Telegram, message_events: &mut EventWriter<MessageEvent>) {
        message_events.send(MessageEvent {
            receiver: telegram.receiver,
            sender: telegram.sender,
            message: telegram.message,
        });
    }

    pub fn dispatch_message(
        &mut self,
        sender: Entity,
        receiver: Entity,
        message: Message,
        message_events: &mut EventWriter<MessageEvent>,
    ) {
        let now = Utc::now().timestamp_millis();
        let telegram = Telegram::new(now, sender, receiver, message);

        self.discharge(telegram, message_events);
    }

    pub fn defer_dispatch_message(
        &mut self,
        sender: Entity,
        receiver: Entity,
        message: Message,
        delay_seconds: f64,
    ) {
        let now = Utc::now().timestamp_millis();
        let telegram = Telegram::new(
            now + (delay_seconds * 1000.0) as i64,
            sender,
            receiver,
            message,
        );

        self.queue.push(telegram);
    }
}
