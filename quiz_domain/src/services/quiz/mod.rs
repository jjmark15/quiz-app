use crate::models::quiz::question::{ModelID, QuestionSetImpl, QuestionSetInterface};

pub trait QuizServiceInterface<'a, QuestionSet>
where
    QuestionSet: QuestionSetInterface<'a>,
    QuestionSet::ID: ModelID<'a>,
{
    fn get_example_question_set() -> QuestionSet {
        QuestionSet::with_id(
            QuestionSet::ID::default(),
            "Example question set title".to_string(),
        )
    }
}

pub struct QuizServiceImpl;

impl QuizServiceInterface<'_, QuestionSetImpl> for QuizServiceImpl {}
