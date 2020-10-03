use quiz_domain::{ModelId, QuestionSet};

pub struct QuestionSetDto {
    id: String,
    name: String,
}

impl QuestionSetDto {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn new(id: String, name: String) -> Self {
        QuestionSetDto { id, name }
    }
}

impl<'a, QS> From<QS> for QuestionSetDto
where
    QS: QuestionSet,
{
    fn from(question_set: QS) -> Self {
        QuestionSetDto::new(
            question_set.id().value().to_string(),
            question_set.name().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use spectral::prelude::*;

    use quiz_domain::Uuid;

    use crate::quiz_domain::mocks::{MockModelId, MockQuestionSet};

    use super::*;

    fn question_set_dto() -> QuestionSetDto {
        QuestionSetDto::new("id".to_string(), "name".to_string())
    }

    #[test]
    fn returns_id() {
        let under_test = question_set_dto();
        assert_that(&under_test.id()).is_equal_to(&"id".to_string())
    }

    #[test]
    fn returns_name() {
        let under_test = question_set_dto();
        assert_that(&under_test.name()).is_equal_to(&"name".to_string())
    }

    #[test]
    fn maps_from_question_set() {
        let mut mock_question_set = MockQuestionSet::<MockModelId>::default();
        mock_question_set.expect_id().returning(mock_model_id);
        mock_question_set
            .expect_name()
            .return_const("name".to_string());
        let dto: QuestionSetDto = mock_question_set.into();
        assert_that(&dto.id()).is_equal_to(&"00000000-0000-0000-0000-000000000000".to_string());
        assert_that(&dto.name()).is_equal_to(&"name".to_string());
    }

    fn mock_model_id() -> MockModelId {
        let mut mock_model_id = MockModelId::default();
        mock_model_id
            .expect_value()
            .returning(|| Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
        mock_model_id
    }
}
