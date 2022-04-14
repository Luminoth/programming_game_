use bevy::prelude::*;

use crate::game::state::State;

#[derive(Debug)]
pub struct StateEnterEvent<T>
where
    T: State,
{
    entity: Entity,
    state: T,
}

impl<T> StateEnterEvent<T>
where
    T: State,
{
    pub fn new(entity: Entity, state: T) -> Self {
        Self { entity, state }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn state(&self) -> T {
        self.state
    }
}

#[derive(Debug)]
pub struct StateExitEvent<T>
where
    T: State,
{
    entity: Entity,
    state: T,
}

impl<T> StateExitEvent<T>
where
    T: State,
{
    pub fn new(entity: Entity, state: T) -> Self {
        Self { entity, state }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn state(&self) -> T {
        self.state
    }
}
