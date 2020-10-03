use quiz_domain::{ExampleQuizObjectsService, ExampleQuizObjectsServiceImpl};

use crate::application::question_set::QuestionSetDto;

pub trait ApplicationService {
    type ExampleQuizObjectsServiceType: ExampleQuizObjectsService;

    fn get_example_question_set(&self) -> QuestionSetDto;
}

#[derive(Debug)]
pub struct ApplicationServiceImpl {
    example_quiz_objects_service: ExampleQuizObjectsServiceImpl,
}

impl ApplicationServiceImpl {
    pub fn new(example_quiz_objects_service: ExampleQuizObjectsServiceImpl) -> Self {
        ApplicationServiceImpl {
            example_quiz_objects_service,
        }
    }
}

impl ApplicationService for ApplicationServiceImpl {
    type ExampleQuizObjectsServiceType = ExampleQuizObjectsServiceImpl;

    fn get_example_question_set(&self) -> QuestionSetDto {
        self.example_quiz_objects_service
            .get_example_question_set()
            .into()
    }
}

#[cfg(test)]
mockall::mock! {
    pub ApplicationService {}

    trait ApplicationService {
        type ExampleQuizObjectsServiceType = ExampleQuizObjectsServiceImpl;

        fn get_example_question_set(&self) -> QuestionSetDto;
    }
}
