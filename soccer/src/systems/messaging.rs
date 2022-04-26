use bevy::prelude::*;

use crate::events::messaging::DispatchedMessageEvent;
use crate::resources::messaging::{MessageDispatcher, MessageEvent};

pub fn update<T>(
    mut message_dispatcher: ResMut<MessageDispatcher<T>>,
    mut message_events: EventWriter<DispatchedMessageEvent<T>>,
) where
    T: MessageEvent + Send + Sync + 'static,
{
    message_dispatcher.dispatch_deferred_messages(&mut message_events);
}
