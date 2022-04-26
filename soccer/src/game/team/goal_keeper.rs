use crate::events::messaging::*;
use crate::resources::messaging::*;

#[derive(Debug, PartialEq, Eq)]
pub enum GoalKeeperMessage {
    GoHome,
    ReceiveBall,
}

impl MessageEvent for GoalKeeperMessage {}

pub type GoalKeeperMessageDispatcher = MessageDispatcher<GoalKeeperMessage>;
pub type GoalKeeperDispatchedMessageEvent = DispatchedMessageEvent<GoalKeeperMessage>;
