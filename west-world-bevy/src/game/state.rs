use crate::components::state::StateMachine;

pub trait State: Default + Copy {
    fn execute_global(state_machine: &mut StateMachine<Self>);

    fn enter(self, state_machine: &mut StateMachine<Self>);

    fn execute(self, state_machine: &mut StateMachine<Self>);

    fn exit(self, state_machine: &mut StateMachine<Self>);
}
