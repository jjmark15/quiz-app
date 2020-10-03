use std::fmt::Debug;
use std::str::FromStr;

pub use uuid::Uuid;

pub trait ModelId {
    fn value(&self) -> Uuid;

    fn new(value: Uuid) -> Self;

    fn random() -> Self;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ModelIDWithUUID {
    id: Uuid,
}

impl ModelId for ModelIDWithUUID {
    fn value(&self) -> Uuid {
        self.id
    }
    fn new(value: Uuid) -> Self {
        ModelIDWithUUID { id: value }
    }

    fn random() -> Self {
        ModelIDWithUUID::new(Uuid::new_v4())
    }
}

impl Default for ModelIDWithUUID {
    fn default() -> Self {
        ModelIDWithUUID::new(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap())
    }
}
