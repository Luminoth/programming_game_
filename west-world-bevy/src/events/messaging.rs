use bevy::prelude::*;

use crate::game::messaging::Message;

#[derive(Debug)]
pub struct MessageEvent {
    pub receiver: Entity,

    pub sender: Entity,
    pub message: Message,
}
