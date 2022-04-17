use bevy::prelude::*;

use crate::resources::messaging::*;

pub fn update(
    mut message_dispatcher: ResMut<MessageDispatcher>,
    mut message_events: EventWriter<DispatchedMessageEvent>,
) {
    message_dispatcher.dispatch_deferred_messages(&mut message_events);
}
