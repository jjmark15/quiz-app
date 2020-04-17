use crate::domain::models::quiz::question::{ModelIDInterface, QuestionSetInterface};

pub trait QuizServiceInterface<
    'a,
    ID: ModelIDInterface<'a>,
    QuestionSet: QuestionSetInterface<'a, ID>,
>
{
    fn get_example_question_set() -> QuestionSet {
        QuestionSet::with_name("Example question set title".to_string())
    }
}
