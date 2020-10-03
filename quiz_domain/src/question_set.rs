use std::fmt::Debug;

use crate::id::{ModelId, ModelIDWithUUID};

pub trait QuestionSet {
    type QuestionSetId: ModelId;

    fn id(&self) -> Self::QuestionSetId;

    fn name(&self) -> &String;

    fn with_id(id: Self::QuestionSetId, name: String) -> Self;

    fn new(name: String) -> Self;
}

#[derive(Debug, Clone)]
pub struct QuestionSetImpl {
    id: ModelIDWithUUID,
    name: String,
}

impl QuestionSet for QuestionSetImpl {
    type QuestionSetId = ModelIDWithUUID;

    fn id(&self) -> Self::QuestionSetId {
        self.id.clone()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn with_id(id: ModelIDWithUUID, name: String) -> Self {
        QuestionSetImpl { id, name }
    }

    fn new(name: String) -> Self {
        Self::with_id(Self::QuestionSetId::random(), name)
    }
}
