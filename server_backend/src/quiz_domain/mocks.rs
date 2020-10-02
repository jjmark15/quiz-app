use mockall::mock;

use quiz_domain::{ExampleQuizObjectsService, ModelID, QuestionSetInterface};

mock! {
    pub ExampleQuizObjectsService<MID, QS>
    where
        MID: 'static +  ModelID,
        QS: 'static +  QuestionSetInterface<ID=MID>,
    {}

    trait ExampleQuizObjectsService {
        type QuestionSetID = MID;
        type QuestionSet = QS;

        fn get_example_question_set(&self) -> QS;
    }
}
