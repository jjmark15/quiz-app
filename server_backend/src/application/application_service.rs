use quiz_domain::{ExampleQuizObjectsService, ExampleQuizObjectsServiceImpl};

pub trait ApplicationService {
    type ExampleQuizObjectsServiceType: ExampleQuizObjectsService;

    fn get_example_question_set(
        &self,
    ) -> <Self::ExampleQuizObjectsServiceType as ExampleQuizObjectsService>::QuestionSetType;
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

    fn get_example_question_set(
        &self,
    ) -> <Self::ExampleQuizObjectsServiceType as ExampleQuizObjectsService>::QuestionSetType {
        self.example_quiz_objects_service.get_example_question_set()
    }
}

#[cfg(test)]
mockall::mock! {
    pub ApplicationService {}

    trait ApplicationService {
        type ExampleQuizObjectsServiceType = ExampleQuizObjectsServiceImpl;

        fn get_example_question_set(&self) -> quiz_domain::QuestionSetImpl;
    }
}
