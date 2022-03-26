#![allow(non_snake_case)]

use rand::Rng;
use tracing::info;

use crate::entity::Entity;

const BATHROOM_CHANCE: f32 = 0.1;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GlobalState {
    WifeGlobalState,
}

impl GlobalState {
    fn enter(
        state: Self,
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        match state {
            Self::WifeGlobalState => Self::WifeGlobalState_enter(entity, state_machine, wife),
        }
    }

    fn execute(
        state: Self,
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        match state {
            Self::WifeGlobalState => Self::WifeGlobalState_execute(entity, state_machine, wife),
        }
    }

    fn exit(
        state: Self,
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        match state {
            Self::WifeGlobalState => Self::WifeGlobalState_exit(entity, state_machine, wife),
        }
    }
}

impl GlobalState {
    fn WifeGlobalState_enter(_entity: &Entity, _: &mut StateMachine, _wife: &mut WifeComponents) {}

    fn WifeGlobalState_execute(
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < BATHROOM_CHANCE {
            state_machine.change_state(entity, State::VisitBathroom, wife)
        }
    }

    fn WifeGlobalState_exit(_entity: &Entity, _: &mut StateMachine, _wife: &mut WifeComponents) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    DoHouseWork,
    VisitBathroom,
}

impl State {
    fn enter(
        state: Self,
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        match state {
            Self::DoHouseWork => Self::DoHouseWork_enter(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_enter(entity, state_machine, wife),
        }
    }

    fn execute(
        state: Self,
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        match state {
            Self::DoHouseWork => Self::DoHouseWork_execute(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_execute(entity, state_machine, wife),
        }
    }

    fn exit(
        state: Self,
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        match state {
            Self::DoHouseWork => Self::DoHouseWork_exit(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_exit(entity, state_machine, wife),
        }
    }
}

impl State {
    fn DoHouseWork_enter(_entity: &Entity, _: &mut StateMachine, _wife: &mut WifeComponents) {}

    fn DoHouseWork_execute(
        entity: &Entity,
        _state_machine: &mut StateMachine,
        _wife: &mut WifeComponents,
    ) {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..2) {
            0 => info!("{}: Moppin' the floor", entity.name()),
            1 => info!("{}: Washin' the dishes", entity.name()),
            2 => info!("{}: Makin' the bed", entity.name()),
            _ => unreachable!(),
        }
    }

    fn DoHouseWork_exit(_entity: &Entity, _: &mut StateMachine, _: &mut WifeComponents) {}
}

impl State {
    fn VisitBathroom_enter(entity: &Entity, _: &mut StateMachine, _wife: &mut WifeComponents) {
        info!(
            "{}: Walkin' to the can. Need to powda mah pretty li'lle nose",
            entity.name()
        );
    }

    fn VisitBathroom_execute(
        entity: &Entity,
        state_machine: &mut StateMachine,
        wife: &mut WifeComponents,
    ) {
        info!("{}: Ahhhhhh! Sweet relief!", entity.name(),);

        state_machine.revert_to_previous_state(entity, wife);
    }

    fn VisitBathroom_exit(entity: &Entity, _: &mut StateMachine, _: &mut WifeComponents) {
        info!("{}: Leavin' the Jon", entity.name());
    }
}

#[derive(Debug)]
struct StateMachine {
    global_state: GlobalState,

    current_state: State,
    previous_state: Option<State>,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            global_state: GlobalState::WifeGlobalState,
            current_state: State::DoHouseWork,
            previous_state: None,
        }
    }
}

impl StateMachine {
    fn update(&mut self, entity: &Entity, miner: &mut WifeComponents) {
        GlobalState::execute(self.global_state, entity, self, miner);

        State::execute(self.current_state, entity, self, miner);
    }

    fn change_state(&mut self, entity: &Entity, new_state: State, miner: &mut WifeComponents) {
        self.previous_state = Some(self.current_state);

        State::exit(self.current_state, entity, self, miner);

        self.current_state = new_state;

        State::enter(self.current_state, entity, self, miner);
    }

    fn revert_to_previous_state(&mut self, entity: &Entity, miner: &mut WifeComponents) {
        self.change_state(entity, self.previous_state.unwrap(), miner);
    }
}

#[derive(Debug, Default)]
struct WifeComponents {}

impl WifeComponents {
    fn update(&mut self) {}
}

#[derive(Debug)]
pub struct Wife {
    entity: Entity,
    state_machine: StateMachine,
    components: WifeComponents,
}

impl Wife {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            entity: Entity::new(name),
            state_machine: StateMachine::default(),
            components: WifeComponents::default(),
        }
    }

    pub fn update(&mut self) {
        self.components.update();

        self.state_machine
            .update(&self.entity, &mut self.components);
    }
}
