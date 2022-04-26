use crate::events::messaging::*;
use crate::resources::messaging::*;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldPlayerMessage {
    SupportAttacker,
    GoHome,
    ReceiveBall,
    PassToMe,
    Wait,
}

impl MessageEvent for FieldPlayerMessage {}

pub type FieldPlayerMessageDispatcher = MessageDispatcher<FieldPlayerMessage>;
pub type FieldPlayerDispatchedMessageEvent = DispatchedMessageEvent<FieldPlayerMessage>;
