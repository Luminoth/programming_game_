use bevy::prelude::*;

use crate::events::messaging::MessageEvent;
use crate::resources::messaging::MessageDispatcher;

pub fn update(
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut message_events: EventWriter<MessageEvent>,
) {
    message_dispatcher.dispatch_deferred_messages(&mut message_events);
}
