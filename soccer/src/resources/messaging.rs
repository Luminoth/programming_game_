use std::cmp::Ordering;
use std::collections::BinaryHeap;

use bevy::prelude::*;
use chrono::prelude::*;

use crate::events::messaging::DispatchedMessageEvent;

pub trait MessageEvent: Eq {}

#[derive(Debug)]
struct Telegram<T>
where
    T: MessageEvent,
{
    dispatch_time: i64,

    pub receiver: Option<Entity>,

    pub message: T,
}

impl<T> Eq for Telegram<T> where T: MessageEvent {}

impl<T> PartialEq for Telegram<T>
where
    T: MessageEvent,
{
    fn eq(&self, other: &Self) -> bool {
        self.dispatch_time == other.dispatch_time
            && self.receiver == other.receiver
            && self.message == other.message
    }
}

impl<T> PartialOrd for Telegram<T>
where
    T: MessageEvent,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Telegram<T>
where
    T: MessageEvent,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: this may need to be reversed
        // so that we get a time-ordered min-heap?
        self.dispatch_time.cmp(&other.dispatch_time)
    }
}

impl<T> Telegram<T>
where
    T: MessageEvent,
{
    fn new(dispatch_time: i64, receiver: Option<Entity>, message: T) -> Self {
        Self {
            dispatch_time,
            receiver,
            message,
        }
    }
}

pub struct MessageDispatcher<T>
where
    T: MessageEvent,
{
    queue: BinaryHeap<Telegram<T>>,
}

impl<T> Default for MessageDispatcher<T>
where
    T: MessageEvent,
{
    fn default() -> Self {
        Self {
            queue: BinaryHeap::default(),
        }
    }
}

impl<T> MessageDispatcher<T>
where
    T: MessageEvent + Send + Sync + 'static,
{
    pub fn dispatch_deferred_messages(
        &mut self,
        message_events: &mut EventWriter<DispatchedMessageEvent<T>>,
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
        telegram: Telegram<T>,
        message_events: &mut EventWriter<DispatchedMessageEvent<T>>,
    ) {
        message_events.send(DispatchedMessageEvent {
            receiver: telegram.receiver,
            message: telegram.message,
        });
    }

    pub fn dispatch_message(&mut self, receiver: Option<Entity>, message: T) {
        // we always defer so that entities sending messages (as events)
        // in response to events can work
        self.defer_dispatch_message(receiver, message, 0.0);
    }

    pub fn defer_dispatch_message(
        &mut self,
        receiver: Option<Entity>,
        message: T,
        delay_seconds: f64,
    ) {
        let now = Utc::now().timestamp_millis();
        let telegram = Telegram::new(now + (delay_seconds * 1000.0) as i64, receiver, message);

        self.queue.push(telegram);
    }
}
