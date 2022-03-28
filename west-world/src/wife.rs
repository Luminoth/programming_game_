#![allow(non_snake_case)]

use rand::Rng;
use tracing::info;

use crate::entity::Entity;
use crate::state::{State, StateMachine};

const BATHROOM_CHANCE: f32 = 0.1;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WifeState {
    GlobalState,

    DoHouseWork,
    VisitBathroom,
}

impl State<WifeComponents> for WifeState {
    type StateMachine = WifeStateMachine;

    fn enter(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        wife: &mut WifeComponents,
    ) {
        match self {
            Self::GlobalState => Self::WifeGlobalState_enter(entity, state_machine, wife),
            Self::DoHouseWork => Self::DoHouseWork_enter(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_enter(entity, state_machine, wife),
        }
    }

    fn execute(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        wife: &mut WifeComponents,
    ) {
        match self {
            Self::GlobalState => Self::WifeGlobalState_execute(entity, state_machine, wife),
            Self::DoHouseWork => Self::DoHouseWork_execute(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_execute(entity, state_machine, wife),
        }
    }

    fn exit(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        wife: &mut WifeComponents,
    ) {
        match self {
            Self::GlobalState => Self::WifeGlobalState_exit(entity, state_machine, wife),
            Self::DoHouseWork => Self::DoHouseWork_exit(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_exit(entity, state_machine, wife),
        }
    }
}

impl WifeState {
    fn WifeGlobalState_enter(
        _entity: &Entity,
        _: &mut WifeStateMachine,
        _wife: &mut WifeComponents,
    ) {
    }

    fn WifeGlobalState_execute(
        entity: &Entity,
        state_machine: &mut WifeStateMachine,
        wife: &mut WifeComponents,
    ) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < BATHROOM_CHANCE {
            state_machine.change_state(entity, Self::VisitBathroom, wife)
        }
    }

    fn WifeGlobalState_exit(
        _entity: &Entity,
        _: &mut WifeStateMachine,
        _wife: &mut WifeComponents,
    ) {
    }
}

impl WifeState {
    fn DoHouseWork_enter(_entity: &Entity, _: &mut WifeStateMachine, _wife: &mut WifeComponents) {}

    fn DoHouseWork_execute(
        entity: &Entity,
        _state_machine: &mut WifeStateMachine,
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

    fn DoHouseWork_exit(_entity: &Entity, _: &mut WifeStateMachine, _: &mut WifeComponents) {}
}

impl WifeState {
    fn VisitBathroom_enter(entity: &Entity, _: &mut WifeStateMachine, _wife: &mut WifeComponents) {
        info!(
            "{}: Walkin' to the can. Need to powda mah pretty li'lle nose",
            entity.name()
        );
    }

    fn VisitBathroom_execute(
        entity: &Entity,
        state_machine: &mut WifeStateMachine,
        wife: &mut WifeComponents,
    ) {
        info!("{}: Ahhhhhh! Sweet relief!", entity.name(),);

        state_machine.revert_to_previous_state(entity, wife);
    }

    fn VisitBathroom_exit(entity: &Entity, _: &mut WifeStateMachine, _: &mut WifeComponents) {
        info!("{}: Leavin' the Jon", entity.name());
    }
}

#[derive(Debug)]
struct WifeStateMachine {
    global_state: WifeState,

    current_state: WifeState,
    previous_state: Option<WifeState>,
}

impl Default for WifeStateMachine {
    fn default() -> Self {
        Self {
            global_state: WifeState::GlobalState,
            current_state: WifeState::DoHouseWork,
            previous_state: None,
        }
    }
}

impl StateMachine<WifeComponents> for WifeStateMachine {
    type State = WifeState;

    fn update(&mut self, entity: &Entity, wife: &mut WifeComponents) {
        self.global_state.execute(entity, self, wife);

        self.current_state.execute(entity, self, wife);
    }

    fn change_state(&mut self, entity: &Entity, new_state: Self::State, wife: &mut WifeComponents) {
        self.previous_state = Some(self.current_state);

        self.current_state.exit(entity, self, wife);

        self.current_state = new_state;

        self.current_state.enter(entity, self, wife);
    }

    fn revert_to_previous_state(&mut self, entity: &Entity, wife: &mut WifeComponents) {
        self.change_state(entity, self.previous_state.unwrap(), wife);
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
    state_machine: WifeStateMachine,
    components: WifeComponents,
}

impl Wife {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            entity: Entity::new(name),
            state_machine: WifeStateMachine::default(),
            components: WifeComponents::default(),
        }
    }

    pub fn update(&mut self) {
        self.components.update();

        self.state_machine
            .update(&self.entity, &mut self.components);
    }
}
