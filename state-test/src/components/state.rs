use bevy::prelude::*;

// base state trait
// sparse storage because these get added and removed frequently
pub trait StateComponent: Component<Storage = bevy::ecs::component::SparseStorage> {}

macro_rules! impl_state_machine {
    ($name:ident, $($states:ident),+) => {
        paste::paste! {
            // base state type component trait
            pub trait [<$name StateComponent>]: $crate::components::state::StateComponent {}

            // state marker components
            $(
                // state enter
                #[derive(Debug, Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Enter>];

                impl $crate::components::state::StateComponent for [<$name State $states Enter>] {}

                impl [<$name StateComponent>] for [<$name State $states Enter>] {}

                // state exit
                #[derive(Debug, Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Exit>];

                impl $crate::components::state::StateComponent for [<$name State $states Exit>] {}

                impl [<$name StateComponent>] for [<$name State $states Exit>] {}

                // state execute
                #[derive(Debug, Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Execute>];

                impl $crate::components::state::StateComponent for [<$name State $states Execute>] {}

                impl [<$name StateComponent>] for [<$name State $states Execute>] {}
            )*

            // enum for working with states outside of component management
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum [<$name States>] {
                $(
                    $states,
                )*
            }

            impl [<$name States>] {
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
            #[derive(Debug, Component)]
            pub struct [<$name StateMachine>] {
                current_state: [<$name States>],
                previous_state: Option<[<$name States>]>,
            }

            impl [<$name StateMachine>] {
                // adds a state machine to the entity
                pub fn insert(commands: &mut bevy::ecs::system::EntityCommands, starting_state: [<$name States>]) {
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
                    commands: &mut Commands,
                    entity: Entity,
                    new_state: [<$name States>],
                ) {
                    let mut entity = commands.entity(entity);

                    // remove all of the state components
                    [<$name States>]::clear(&mut entity);

                    // insert the state exit component
                    self.current_state.insert_exit(&mut entity);

                    self.previous_state = Some(self.current_state);
                    self.current_state = new_state;
                }

                #[allow(dead_code)]
                pub fn revert_to_previous_state(
                    &mut self,
                    commands: &mut Commands,
                    entity: Entity,
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
                    mut commands: Commands,
                    query: Query<Entity, With<[<$name State $states Enter>]>>
                ) {
                    for entity in query.iter() {
                        let mut entity = commands.entity(entity);
                        entity.remove::<[<$name State $states Enter>]>();
                        entity.insert([<$name State $states Execute>]);
                    }
                }

                #[allow(non_snake_case)]
                fn [<$name _state_ $states _exit_advance>](
                    mut commands: Commands,
                    query: Query<(Entity, &[<$name StateMachine>]), With<[<$name State $states Exit>]>>
                ) {
                    for (entity,  state_machine) in query.iter() {
                        let mut entity = commands.entity(entity);
                        entity.remove::<[<$name State $states Exit>]>();
                        state_machine.current_state.insert_enter(&mut entity);
                    }
                }
            )*

            // plugin
            pub struct [<$name StateMachinePlugin>];

            impl Plugin for [<$name StateMachinePlugin>] {
                fn build(&self, app: &mut App) {
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
