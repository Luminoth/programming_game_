use crate::events::messaging::*;
use crate::resources::messaging::*;

use crate::game::team::Team;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldPlayerMessage {
    SupportAttacker,
    GoHome(Team),
    ReceiveBall,
    PassToMe,
    Wait,
}

impl MessageEvent for FieldPlayerMessage {}

pub type FieldPlayerMessageDispatcher = MessageDispatcher<FieldPlayerMessage>;
pub type FieldPlayerDispatchedMessageEvent = DispatchedMessageEvent<FieldPlayerMessage>;
