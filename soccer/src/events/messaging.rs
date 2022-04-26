use bevy::prelude::*;

use crate::resources::messaging::MessageEvent;

#[derive(Debug)]
pub struct DispatchedMessageEvent<T>
where
    T: MessageEvent,
{
    pub receiver: Option<Entity>,
    pub message: T,
}
