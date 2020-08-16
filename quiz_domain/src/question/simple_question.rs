use crate::{Answer, ModelIDWithUUID, Question};

#[derive(Debug, Clone)]
pub struct SimpleQuestion<A>
where
    A: Answer,
{
    id: ModelIDWithUUID,
    question_set_id: ModelIDWithUUID,
    query_message: String,
    answer: A,
}

impl<A> SimpleQuestion<A>
where
    A: Answer,
{
    pub fn new(
        id: ModelIDWithUUID,
        question_set_id: ModelIDWithUUID,
        query_message: String,
        answer: A,
    ) -> Self {
        SimpleQuestion {
            id,
            question_set_id,
            query_message,
            answer,
        }
    }
}

impl<A> Question for SimpleQuestion<A>
where
    A: Answer,
{
    type QuestionID = ModelIDWithUUID;
    type QuestionSetID = ModelIDWithUUID;
    type QuestionAnswer = A;

    fn id(&self) -> &Self::QuestionID {
        &self.id
    }

    fn question_set_id(&self) -> &Self::QuestionSetID {
        &self.question_set_id
    }

    fn query_message(&self) -> &String {
        &self.query_message
    }

    fn answer(&self) -> &Self::QuestionAnswer {
        &self.answer
    }

    fn answered_by(&self, answer: &Self::QuestionAnswer) -> bool {
        self.answer.satisfied_by(answer)
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::MockAnswer;

    use super::*;

    fn simple_question<ANSWER: Answer>(answer: ANSWER) -> SimpleQuestion<ANSWER> {
        SimpleQuestion::new(
            ModelIDWithUUID::default(),
            ModelIDWithUUID::default(),
            "Query message".to_string(),
            answer,
        )
    }

    fn default_answer() -> MockAnswer {
        let mut mock = MockAnswer::default();
        mock.expect_satisfied_by().return_const(true);
        mock
    }

    fn unsatisfied_answer() -> MockAnswer {
        let mut mock = MockAnswer::default();
        mock.expect_satisfied_by().return_const(false);
        mock
    }

    #[test]
    fn returns_its_id() {
        asserting("returns its id")
            .that(&simple_question(default_answer()).id())
            .is_equal_to(&ModelIDWithUUID::default());
    }

    #[test]
    fn returns_its_question_set_id() {
        asserting("returns its question set id")
            .that(&simple_question(default_answer()).question_set_id())
            .is_equal_to(&ModelIDWithUUID::default());
    }

    #[test]
    fn returns_a_query_message() {
        asserting("returns a query message")
            .that(&simple_question(default_answer()).query_message())
            .is_equal_to(&"Query message".to_string())
    }

    #[test]
    fn returns_its_answer() {
        let mut answer = default_answer();
        answer.expect_eq().return_const(true);
        asserting("returns it's answer")
            .that(&simple_question(answer).answer())
            .is_equal_to(&default_answer());
    }

    #[test]
    fn answered_by_correct_answer() {
        asserting("answered by correct answer")
            .that(&simple_question(default_answer()).answered_by(&default_answer()))
            .is_true()
    }

    #[test]
    fn not_answered_by_incorrect_answer() {
        let incorrect_answer = default_answer();
        asserting("not answered by incorrect answer")
            .that(&simple_question(unsatisfied_answer()).answered_by(&incorrect_answer))
            .is_false()
    }
}
