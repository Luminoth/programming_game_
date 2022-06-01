use std::sync::atomic::{AtomicI64, Ordering};

use mlua::{UserData, UserDataMethods};

pub type EntityId = i64;

#[derive(Debug, Clone)]
pub struct Entity {
    id: EntityId,
    name: String,
}

static NEXT_ID: AtomicI64 = AtomicI64::new(1);

impl Entity {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            name: name.into(),
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl UserData for Entity {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("id", |_, this, ()| Ok(this.id()));
        methods.add_method("name", |_, this, ()| Ok(this.name.clone()));
    }
}
