use std::fmt::Debug;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub trait ModelID<'a>: Eq + Deserialize<'a> + Serialize + Clone + Debug + Default {
    fn value(&self) -> uuid::Uuid;

    fn new(value: uuid::Uuid) -> Self;

    fn random() -> Self;
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct ModelIDImpl {
    id: uuid::Uuid,
}

impl ModelID<'_> for ModelIDImpl {
    fn value(&self) -> uuid::Uuid {
        self.id
    }
    fn new(value: uuid::Uuid) -> Self {
        ModelIDImpl { id: value }
    }

    fn random() -> Self {
        ModelIDImpl::new(uuid::Uuid::new_v4())
    }
}

impl Default for ModelIDImpl {
    fn default() -> Self {
        ModelIDImpl::new(uuid::Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap())
    }
}

pub trait QuestionSetInterface<'a>: Debug + Deserialize<'a> + Serialize + Clone {
    type ID: ModelID<'a>;

    fn id(&self) -> Self::ID;

    fn name(&self) -> &String;

    fn with_id(id: Self::ID, name: String) -> Self;

    fn new(name: String) -> Self {
        Self::with_id(Self::ID::random(), name)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuestionSetImpl {
    id: ModelIDImpl,
    name: String,
}

impl QuestionSetInterface<'_> for QuestionSetImpl {
    type ID = ModelIDImpl;

    fn id(&self) -> Self::ID {
        self.id.clone()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn with_id(id: ModelIDImpl, name: String) -> Self {
        QuestionSetImpl { id, name }
    }
}
