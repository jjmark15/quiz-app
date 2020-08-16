use std::fmt::Debug;
use std::str::FromStr;

pub trait ModelID: Eq + for<'a> Clone + Debug + Default {
    fn value(&self) -> uuid::Uuid;

    fn new(value: uuid::Uuid) -> Self;

    fn random() -> Self;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ModelIDWithUUID {
    id: uuid::Uuid,
}

impl ModelID for ModelIDWithUUID {
    fn value(&self) -> uuid::Uuid {
        self.id
    }
    fn new(value: uuid::Uuid) -> Self {
        ModelIDWithUUID { id: value }
    }

    fn random() -> Self {
        ModelIDWithUUID::new(uuid::Uuid::new_v4())
    }
}

impl Default for ModelIDWithUUID {
    fn default() -> Self {
        ModelIDWithUUID::new(uuid::Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap())
    }
}
