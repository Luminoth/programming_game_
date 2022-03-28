use crate::entity::{Entity, EntityId};
use crate::messaging::Message;

pub trait State<T> {
    type StateMachine: StateMachine<T>;

    fn enter(self, entity: &Entity, state_machine: &mut Self::StateMachine, data: &mut T);

    fn execute(self, entity: &Entity, state_machine: &mut Self::StateMachine, data: &mut T);

    fn exit(self, entity: &Entity, state_machine: &mut Self::StateMachine, data: &mut T);

    fn on_message(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        data: &mut T,
        sender: EntityId,
        message: &Message,
    ) -> bool;
}

// TODO: this whole interface could be implemented here
// if we instead had trait methods for getting at the global / current / previous states
pub trait StateMachine<T> {
    type State: State<T>;

    fn update(&mut self, entity: &Entity, data: &mut T);

    fn change_state(&mut self, entity: &Entity, new_state: Self::State, data: &mut T);

    fn revert_to_previous_state(&mut self, entity: &Entity, data: &mut T);

    fn handle_message(&mut self, entity: &Entity, data: &mut T, sender: EntityId, message: Message);
}
