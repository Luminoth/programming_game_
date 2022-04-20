macro_rules! impl_state_machine {
    ($name:ident, $($states:ident),+) => {
        paste::paste! {
            // base state component trait
            // sparse storage because these get added and removed frequently
            pub trait [<$name StateComponent>]: Component<Storage = bevy::ecs::component::SparseStorage> {}

            // state components
            $(
                // state enter
                #[derive(Debug, Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Enter>];

                impl [<$name StateComponent>] for [<$name State $states Enter>] {}

                // state exit
                #[derive(Debug, Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Exit>];

                impl [<$name StateComponent>] for [<$name State $states Exit>] {}

                // state execute
                #[derive(Debug, Component)]
                #[component(storage = "SparseSet")]
                pub struct [<$name State $states Execute>];

                impl [<$name StateComponent>] for [<$name State $states Execute>] {}
            )*

            // enum for working with states outside of component management
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum [<$name States>] {
                $(
                    $states,
                )*
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
                    // insert the state machine
                    commands.insert(Self {
                        current_state: starting_state,
                        previous_state: None,
                    });

                    // insert the starting state component
                    match starting_state {
                        $(
                            [<$name States>]::$states => commands.insert([<$name State $states Execute>]),
                        )*
                    };
                }

                pub fn change_state(
                    &mut self,
                    commands: &mut Commands,
                    entity: Entity,
                    new_state: [<$name States>],
                ) {
                    self.previous_state = Some(self.current_state);
                    self.current_state = new_state;

                    let mut entity = commands.entity(entity);

                    // remove all of the state components
                    $(
                        entity.remove::<[<$name State $states Enter>]>();
                        entity.remove::<[<$name State $states Exit>]>();
                        entity.remove::<[<$name State $states Execute>]>();
                    )*

                    // insert the new state enter component
                    match self.current_state {
                        $(
                            [<$name States>]::$states => entity.insert([<$name State $states Enter>]),
                        )*
                    };
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
                    query: Query<Entity, With<[<$name State $states Exit>]>>
                ) {
                    for entity in query.iter() {
                        let mut entity = commands.entity(entity);
                        entity.remove::<[<$name State $states Exit>]>();
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
                            .add_system([<$name _state_ $states _exit_advance>]);
                    )*
                }
            }
        }
    };
}

pub(crate) use impl_state_machine;
