use crate::domain::quiz::question::QuestionSet;

pub(crate) struct QuizService;

impl QuizService {
    pub(crate) fn get_example_question_set() -> QuestionSet {
        QuestionSet::new("Example question set title".to_string(), 0)
    }
}
