use crate::models::quiz::{ModelID, ModelIDImpl, QuestionSetImpl, QuestionSetInterface};

pub trait QuizServiceInterface<'a> {
    type QuestionSetID: ModelID<'a>;
    type QuestionSet: QuestionSetInterface<'a, ID = Self::QuestionSetID>;

    fn get_example_question_set() -> Self::QuestionSet {
        Self::QuestionSet::with_id(
            Self::QuestionSetID::default(),
            "Example question set title".to_string(),
        )
    }
}

pub struct QuizServiceImpl;

impl QuizServiceInterface<'_> for QuizServiceImpl {
    type QuestionSetID = ModelIDImpl;
    type QuestionSet = QuestionSetImpl;
}
