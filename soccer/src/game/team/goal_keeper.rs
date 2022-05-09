use crate::events::messaging::*;
use crate::resources::messaging::*;

#[derive(Debug, PartialEq)]
pub enum GoalKeeperMessage {
    //GoHome,
//ReceiveBall,
}

impl Eq for GoalKeeperMessage {}

impl MessageEvent for GoalKeeperMessage {}

pub type GoalKeeperMessageDispatcher = MessageDispatcher<GoalKeeperMessage>;
pub type GoalKeeperDispatchedMessageEvent = DispatchedMessageEvent<GoalKeeperMessage>;
