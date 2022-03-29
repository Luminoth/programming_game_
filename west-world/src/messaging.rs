use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

use chrono::prelude::*;
use tracing::debug;

use crate::entity::{Entity, EntityId};

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    HiHoneyImHome,
    StewIsReady,
}

pub trait MessageReceiver {
    fn receive_message(&mut self, sender: EntityId, message: Message);
}

#[derive(Debug)]
struct Telegram {
    dispatch_time: i64,

    sender: EntityId,
    receiver: EntityId,

    message: Message,
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
    fn new(dispatch_time: i64, sender: EntityId, receiver: EntityId, message: Message) -> Self {
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
    receivers: RefCell<HashMap<EntityId, Rc<RefCell<dyn MessageReceiver>>>>,

    queue: RefCell<BinaryHeap<Telegram>>,
}

impl MessageDispatcher {
    pub fn register_message_receiver(
        &self,
        entity: &Entity,
        receiver: Rc<RefCell<dyn MessageReceiver>>,
    ) {
        self.receivers.borrow_mut().insert(entity.id(), receiver);
    }

    pub fn dispatch_deferred_messages(&self) {
        let now = Utc::now().timestamp_millis();

        debug!("now: {}, queue: {:?}", now, self.queue.borrow());

        loop {
            if let Some(telegram) = self.queue.borrow().peek() {
                if telegram.dispatch_time > now {
                    return;
                }
            } else {
                return;
            }

            let telegram = self.queue.borrow_mut().pop().unwrap();
            self.discharge(telegram);
        }
    }

    fn discharge(&self, telegram: Telegram) {
        if let Some(receiver) = self.receivers.borrow_mut().get_mut(&telegram.receiver) {
            receiver
                .borrow_mut()
                .receive_message(telegram.sender, telegram.message)
        }
    }

    pub fn dispatch_message(&self, sender: EntityId, receiver: EntityId, message: Message) {
        // we always defer so that entities sending messages to themselves
        // doesn't cause a double mutable borrow on the receiver
        self.defer_dispatch_message(sender, receiver, message, 0.0);
    }

    pub fn defer_dispatch_message(
        &self,
        sender: EntityId,
        receiver: EntityId,
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

        self.queue.borrow_mut().push(telegram);
    }
}
