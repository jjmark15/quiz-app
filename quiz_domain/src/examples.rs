use crate::{ModelID, ModelIDWithUUID, QuestionSetImpl, QuestionSetInterface};

pub trait ExampleQuizObjectsService {
    type QuestionSetID: ModelID;
    type QuestionSet: QuestionSetInterface<ID = Self::QuestionSetID>;

    fn get_example_question_set(&self) -> Self::QuestionSet {
        Self::QuestionSet::with_id(
            Self::QuestionSetID::default(),
            "Example question set title".to_string(),
        )
    }
}

#[derive(Debug, Default)]
pub struct ExampleQuizObjectsServiceImpl;

impl ExampleQuizObjectsService for ExampleQuizObjectsServiceImpl {
    type QuestionSetID = ModelIDWithUUID;
    type QuestionSet = QuestionSetImpl;
}

impl ExampleQuizObjectsServiceImpl {
    pub fn new() -> Self {
        ExampleQuizObjectsServiceImpl
    }
}
