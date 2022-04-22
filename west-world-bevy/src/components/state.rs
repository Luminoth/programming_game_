use bevy::prelude::*;

// base state trait
// sparse storage because these get added and removed frequently
pub trait StateComponent: Component<Storage = bevy::ecs::component::SparseStorage> {}

// TODO: we can't use states across stages as noted here: https://bevy-cheatbook.github.io/programming/states.html
// so for now we're stuck with a frame between every state transition step

// state transitions need to happen in separate stages
// so that component changes can be committed
// and everything can happen in a single frame
/*#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StateMachineStage {
    Exit,
    Enter,
}*/

macro_rules! impl_state_machine {
    ($name:ident, $($states:ident),+) => {
        paste::paste! {
            use bevy::prelude::ParallelSystemDescriptorCoercion;

            // base state type component trait
            pub trait [<$name StateComponent>]: $crate::components::state::StateComponent {}

            // state marker components
            $(
                // state enter
                #[derive(Debug, bevy::prelude::Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Enter>];

                impl $crate::components::state::StateComponent for [<$name State $states Enter>] {}

                impl [<$name StateComponent>] for [<$name State $states Enter>] {}

                // state exit
                #[derive(Debug, bevy::prelude::Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Exit>];

                impl $crate::components::state::StateComponent for [<$name State $states Exit>] {}

                impl [<$name StateComponent>] for [<$name State $states Exit>] {}

                // state execute
                #[derive(Debug, bevy::prelude::Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Execute>];

                impl $crate::components::state::StateComponent for [<$name State $states Execute>] {}

                impl [<$name StateComponent>] for [<$name State $states Execute>] {}
            )*

            // enum for working with states outside of component management
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum [<$name State>] {
                $(
                    $states,
                )*
            }

            impl [<$name State>] {
                // remove all of the state markers from an entity
                fn clear(commands: &mut bevy::ecs::system::EntityCommands) {
                    $(
                        commands.remove::<[<$name State $states Enter>]>();
                        commands.remove::<[<$name State $states Exit>]>();
                        commands.remove::<[<$name State $states Execute>]>();
                    )*
                }

                // helpers for adding state markers to entities

                fn insert_enter(&self, commands: &mut bevy::ecs::system::EntityCommands) {
                    match self {
                        $(
                            Self::$states => commands.insert([<$name State $states Enter>]),
                        )*
                    };
                }

                fn insert_exit(&self, commands: &mut bevy::ecs::system::EntityCommands) {
                    match self {
                        $(
                            Self::$states => commands.insert([<$name State $states Exit>]),
                        )*
                    };
                }

                fn insert_execute(&self, commands: &mut bevy::ecs::system::EntityCommands) {
                    match self {
                        $(
                            Self::$states => commands.insert([<$name State $states Execute>]),
                        )*
                    };
                }
            }

            // the state machine
            #[derive(Debug, bevy::prelude::Component)]
            pub struct [<$name StateMachine>] {
                current_state: [<$name State>],
                previous_state: Option<[<$name State>]>,
            }

            impl [<$name StateMachine>] {
                // adds a state machine to the entity
                pub fn insert(commands: &mut bevy::ecs::system::EntityCommands, starting_state: [<$name State>]) {
                    bevy::prelude::debug!("inserting state machine ...");

                    // TODO: is there a way to clean up any pre-existing state machine components?

                    // insert the state machine
                    commands.insert(Self {
                        current_state: starting_state,
                        previous_state: None,
                    });

                    // insert the starting state component
                    starting_state.insert_execute(commands);
                }

                pub fn change_state(
                    &mut self,
                    commands: &mut bevy::prelude::Commands,
                    entity: bevy::prelude::Entity,
                    new_state: [<$name State>],
                ) {
                    bevy::prelude::debug!("changing state ...");

                    let mut entity = commands.entity(entity);

                    // remove all of the state components
                    [<$name State>]::clear(&mut entity);

                    // insert the state exit component
                    self.current_state.insert_exit(&mut entity);

                    self.previous_state = Some(self.current_state);
                    self.current_state = new_state;
                }

                #[allow(dead_code)]
                pub fn revert_to_previous_state(
                    &mut self,
                    commands: &mut bevy::prelude::Commands,
                    entity: bevy::prelude::Entity,
                ) {
                    self.change_state(
                        commands,
                        entity,
                        self.previous_state.unwrap(),
                    );
                }
            }

            // state advancement systems
            // needed to move from enter -> execute and to remove the exit state marker
            $(
                #[allow(non_snake_case)]
                fn [<$name _state_ $states _enter_advance>](
                    mut commands: bevy::prelude::Commands,
                    query: bevy::prelude::Query<bevy::prelude::Entity, bevy::prelude::With<[<$name State $states Enter>]>>
                ) {
                    bevy::prelude::debug!("advancing enter states to execute ...");

                    for entity in query.iter() {
                        let mut entity = commands.entity(entity);
                        entity.remove::<[<$name State $states Enter>]>();
                        entity.insert([<$name State $states Execute>]);
                    }
                }

                #[allow(non_snake_case)]
                fn [<$name _state_ $states _exit_advance>](
                    mut commands: bevy::prelude::Commands,
                    query: bevy::prelude::Query<(bevy::prelude::Entity, &[<$name StateMachine>]), bevy::prelude::With<[<$name State $states Exit>]>>
                ) {
                    bevy::prelude::debug!("advancing exit states to enter ...");

                    for (entity, state_machine) in query.iter() {
                        let mut entity = commands.entity(entity);
                        entity.remove::<[<$name State $states Exit>]>();
                        state_machine.current_state.insert_enter(&mut entity);
                    }
                }
            )*

            // plugin
            pub struct [<$name StateMachinePlugin>];

            impl bevy::prelude::Plugin for [<$name StateMachinePlugin>] {
                fn build(&self, app: &mut bevy::prelude::App) {
                    bevy::prelude::debug!("setting up state machine plugin ...");

                    // exit stage
                    /*app.add_stage_before(bevy::prelude::CoreStage::Update, crate::components::state::StateMachineStage::Exit, bevy::prelude::SystemStage::parallel())
                    $(
                        .add_system([<$name _state_ $states _exit_advance>])
                    )*;*/

                    // enter stage
                    /*app.add_stage_after(crate::components::state::StateMachineStage::Exit, crate::components::state::StateMachineStage::Enter, bevy::prelude::SystemStage::parallel())
                    $(
                        .add_system([<$name _state_ $states _enter_advance>])
                    )*;*/

                    // state advancement
                    $(
                        app.add_system([<$name _state_ $states _enter_advance>])
                            .add_system([<$name _state_ $states _exit_advance>]
                                .after([<$name _state_ $states _enter_advance>])
                            );
                    )*
                }
            }
        }
    };
}

pub(crate) use impl_state_machine;
