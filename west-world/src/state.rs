use crate::entity::Entity;

pub trait State<T> {
    type StateMachine: StateMachine<T>;

    fn enter(self, entity: &Entity, state_machine: &mut Self::StateMachine, data: &mut T);

    fn execute(self, entity: &Entity, state_machine: &mut Self::StateMachine, data: &mut T);

    fn exit(self, entity: &Entity, state_machine: &mut Self::StateMachine, data: &mut T);
}

pub trait StateMachine<T> {
    type State: State<T>;

    fn update(&mut self, entity: &Entity, data: &mut T);

    fn change_state(&mut self, entity: &Entity, new_state: Self::State, data: &mut T);

    fn revert_to_previous_state(&mut self, entity: &Entity, data: &mut T);
}
