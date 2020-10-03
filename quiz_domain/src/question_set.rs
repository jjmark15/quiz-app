use std::fmt::Debug;

use crate::id::{ModelID, ModelIDWithUUID};

pub trait QuestionSet {
    type ID: ModelID;

    fn id(&self) -> Self::ID;

    fn name(&self) -> &String;

    fn with_id(id: Self::ID, name: String) -> Self;

    fn new(name: String) -> Self;
}

#[derive(Debug, Clone)]
pub struct QuestionSetImpl {
    id: ModelIDWithUUID,
    name: String,
}

impl QuestionSet for QuestionSetImpl {
    type ID = ModelIDWithUUID;

    fn id(&self) -> Self::ID {
        self.id.clone()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn with_id(id: ModelIDWithUUID, name: String) -> Self {
        QuestionSetImpl { id, name }
    }

    fn new(name: String) -> Self {
        Self::with_id(Self::ID::random(), name)
    }
}
