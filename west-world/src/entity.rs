use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug)]
pub struct Entity {
    id: i64,
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

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
