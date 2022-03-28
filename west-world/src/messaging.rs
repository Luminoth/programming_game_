use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

use chrono::prelude::*;

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
    receivers: HashMap<EntityId, Rc<RefCell<dyn MessageReceiver>>>,

    queue: BinaryHeap<Telegram>,
}

impl MessageDispatcher {
    pub fn register_message_receiver(
        &mut self,
        entity: &Entity,
        receiver: Rc<RefCell<dyn MessageReceiver>>,
    ) {
        self.receivers.insert(entity.id(), receiver);
    }

    pub fn dispatch_deferred_messages(&mut self) {
        let now = Utc::now().timestamp_millis();

        while let Some(telegram) = self.queue.peek() {
            if telegram.dispatch_time > now {
                break;
            }

            let telegram = self.queue.pop().unwrap();
            self.discharge(telegram);
        }
    }

    fn discharge(&mut self, telegram: Telegram) {
        if let Some(receiver) = self.receivers.get_mut(&telegram.receiver) {
            receiver
                .borrow_mut()
                .receive_message(telegram.sender, telegram.message)
        }
    }

    pub fn dispatch_message(&mut self, sender: EntityId, receiver: EntityId, message: Message) {
        let telegram = Telegram::new(0, sender, receiver, message);
        self.discharge(telegram);
    }

    pub fn defer_dispatch_message(
        &mut self,
        sender: EntityId,
        receiver: EntityId,
        message: Message,
        delay: i64,
    ) {
        let now = Utc::now().timestamp_millis();
        let telegram = Telegram::new(now + delay, sender, receiver, message);

        self.queue.push(telegram)
    }
}
