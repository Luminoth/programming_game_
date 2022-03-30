#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use chrono::prelude::*;
use rand::Rng;
use tracing::info;

use crate::entity::{Entity, EntityId};
use crate::messaging::{Message, MessageDispatcher, MessageReceiver};
use crate::state::{State, StateMachine};

const BATHROOM_CHANCE: f32 = 0.1;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WifeState {
    GlobalState,

    DoHouseWork,
    VisitBathroom,
    CookStew,
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
            Self::GlobalState => (),
            Self::DoHouseWork => Self::DoHouseWork_enter(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_enter(entity, state_machine, wife),
            Self::CookStew => Self::CookStew_enter(entity, state_machine, wife),
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
            Self::CookStew => Self::CookStew_execute(entity, state_machine, wife),
        }
    }

    fn exit(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        wife: &mut WifeComponents,
    ) {
        match self {
            Self::GlobalState => (),
            Self::DoHouseWork => Self::DoHouseWork_exit(entity, state_machine, wife),
            Self::VisitBathroom => Self::VisitBathroom_exit(entity, state_machine, wife),
            Self::CookStew => Self::CookStew_exit(entity, state_machine, wife),
        }
    }

    fn on_message(
        self,
        entity: &Entity,
        state_machine: &mut Self::StateMachine,
        wife: &mut WifeComponents,
        sender: EntityId,
        message: &Message,
    ) -> bool {
        match self {
            Self::GlobalState => {
                Self::WifeGlobalState_on_message(entity, state_machine, wife, sender, message)
            }
            Self::DoHouseWork => false,
            Self::VisitBathroom => false,
            Self::CookStew => {
                Self::CookStew_on_message(entity, state_machine, wife, sender, message)
            }
        }
    }
}

impl WifeState {
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

    fn WifeGlobalState_on_message(
        entity: &Entity,
        state_machine: &mut WifeStateMachine,
        wife: &mut WifeComponents,
        _sender: EntityId,
        message: &Message,
    ) -> bool {
        match message {
            Message::HiHoneyImHome => {
                let now = Utc::now();

                info!("Message handled by {} at time: {}", entity.name(), now);
                info!(
                    "{}: Hi honey. Let me make you some of mah fine country stew",
                    entity.name()
                );

                state_machine.change_state(entity, Self::CookStew, wife);

                true
            }
            _ => false,
        }
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

        match rng.gen_range(0..=2) {
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

impl WifeState {
    fn CookStew_enter(
        entity: &Entity,
        state_machine: &mut WifeStateMachine,
        wife: &mut WifeComponents,
    ) {
        if wife.cooking {
            return;
        }

        info!("{}: Puttin' the stew in the oven", entity.name());

        state_machine
            .message_dispatcher()
            .borrow()
            .defer_dispatch_message(entity.id(), entity.id(), Message::StewIsReady, 1.5);

        wife.cooking = true;
    }

    fn CookStew_execute(
        _entity: &Entity,
        _state_machine: &mut WifeStateMachine,
        _wife: &mut WifeComponents,
    ) {
    }

    fn CookStew_exit(_entity: &Entity, _: &mut WifeStateMachine, _: &mut WifeComponents) {}

    fn CookStew_on_message(
        entity: &Entity,
        state_machine: &mut WifeStateMachine,
        wife: &mut WifeComponents,
        _sender: EntityId,
        message: &Message,
    ) -> bool {
        match message {
            Message::StewIsReady => {
                let now = Utc::now();

                info!("Message received by {} at time: {}", entity.name(), now);
                info!("{}: Stew ready! Let's eat", entity.name());

                state_machine
                    .message_dispatcher()
                    .borrow()
                    .dispatch_message(entity.id(), wife.miner_id.unwrap(), Message::StewIsReady);

                wife.cooking = false;

                state_machine.change_state(entity, Self::DoHouseWork, wife);

                true
            }
            _ => false,
        }
    }
}

struct WifeStateMachine {
    global_state: WifeState,

    current_state: WifeState,
    previous_state: Option<WifeState>,

    message_dispatcher: Rc<RefCell<MessageDispatcher>>,
}

impl WifeStateMachine {
    fn new(message_dispatcher: Rc<RefCell<MessageDispatcher>>) -> Self {
        Self {
            global_state: WifeState::GlobalState,
            current_state: WifeState::DoHouseWork,
            previous_state: None,
            message_dispatcher,
        }
    }

    fn message_dispatcher(&self) -> &Rc<RefCell<MessageDispatcher>> {
        &self.message_dispatcher
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

    fn handle_message(
        &mut self,
        entity: &Entity,
        miner: &mut WifeComponents,
        sender: EntityId,
        message: Message,
    ) {
        if self
            .current_state
            .on_message(entity, self, miner, sender, &message)
        {
            return;
        }

        self.global_state
            .on_message(entity, self, miner, sender, &message);
    }
}

#[derive(Debug, Default)]
struct WifeComponents {
    cooking: bool,
    miner_id: Option<EntityId>,
}

impl WifeComponents {
    fn update(&mut self) {}
}

pub struct Wife {
    entity: Entity,
    state_machine: WifeStateMachine,
    components: WifeComponents,
}

impl Wife {
    pub fn new(
        name: impl Into<String>,
        message_dispatcher: Rc<RefCell<MessageDispatcher>>,
    ) -> Self {
        Self {
            entity: Entity::new(name),
            state_machine: WifeStateMachine::new(message_dispatcher),
            components: WifeComponents::default(),
        }
    }

    pub fn set_miner_id(&mut self, miner_id: EntityId) {
        self.components.miner_id = Some(miner_id);
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn update(&mut self) {
        self.components.update();

        self.state_machine
            .update(&self.entity, &mut self.components);
    }
}

impl MessageReceiver for Wife {
    fn receive_message(&mut self, sender: EntityId, message: Message) {
        self.state_machine
            .handle_message(&self.entity, &mut self.components, sender, message);
    }
}
