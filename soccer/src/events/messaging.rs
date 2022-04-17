use bevy::prelude::*;

use crate::game::messaging::MessageEvent;

#[derive(Debug)]
pub struct DispatchedMessageEvent {
    pub receiver: Option<Entity>,
    pub message: MessageEvent,
}
