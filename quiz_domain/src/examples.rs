use crate::{ModelID, ModelIDWithUUID, QuestionSetImpl, QuestionSetInterface};

pub trait QuizServiceInterface {
    type QuestionSetID: ModelID;
    type QuestionSet: QuestionSetInterface<ID = Self::QuestionSetID>;

    fn get_example_question_set() -> Self::QuestionSet {
        Self::QuestionSet::with_id(
            Self::QuestionSetID::default(),
            "Example question set title".to_string(),
        )
    }
}

pub struct QuizServiceImpl;

impl QuizServiceInterface for QuizServiceImpl {
    type QuestionSetID = ModelIDWithUUID;
    type QuestionSet = QuestionSetImpl;
}
