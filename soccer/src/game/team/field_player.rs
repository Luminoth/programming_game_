use bevy::prelude::*;

use crate::events::messaging::*;
use crate::resources::messaging::*;

#[derive(Debug, PartialEq)]
pub enum FieldPlayerMessage {
    SupportAttacker,
    GoHome,
    ReceiveBall(Vec2),
    PassToMe(Entity, Vec2),
    Wait,
}

impl Eq for FieldPlayerMessage {}

impl MessageEvent for FieldPlayerMessage {}

pub type FieldPlayerMessageDispatcher = MessageDispatcher<FieldPlayerMessage>;
pub type FieldPlayerDispatchedMessageEvent = DispatchedMessageEvent<FieldPlayerMessage>;
