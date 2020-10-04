use serde::{Deserialize, Serialize};

use crate::application::QuestionSetDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionSetResponse {
    id: String,
    name: String,
}

impl QuestionSetResponse {
    pub(crate) fn new(id: String, name: String) -> Self {
        QuestionSetResponse { id, name }
    }

    #[cfg(test)]
    pub(crate) fn id(&self) -> &String {
        &self.id
    }

    #[cfg(test)]
    pub(crate) fn name(&self) -> &String {
        &self.name
    }
}

impl From<QuestionSetDto> for QuestionSetResponse {
    fn from(question_set_dto: QuestionSetDto) -> Self {
        QuestionSetResponse::new(
            question_set_dto.id().to_string(),
            question_set_dto.name().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn maps_from_question_set_dto() {
        let dto = QuestionSetDto::new("id".to_string(), "name".to_string());
        let response: QuestionSetResponse = dto.into();
        assert_that(&response.id()).is_equal_to(&"id".to_string());
        assert_that(&response.name()).is_equal_to(&"name".to_string());
    }
}
