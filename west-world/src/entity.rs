use std::sync::atomic::{AtomicI64, Ordering};

pub type EntityId = i64;

#[derive(Debug)]
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

    pub fn name(&self) -> &str {
        &self.name
    }
}
