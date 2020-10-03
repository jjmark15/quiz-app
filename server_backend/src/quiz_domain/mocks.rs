use mockall::mock;

use quiz_domain::{ExampleQuizObjectsService, ModelID, QuestionSet};

mock! {
    pub ExampleQuizObjectsService<MID, QS>
    where
        MID: 'static +  ModelID + Default,
        QS: 'static +  QuestionSet<ID=MID>,
    {}

    trait ExampleQuizObjectsService {
        type QuestionSetID = MID;
        type QuestionSetType = QS;

        fn get_example_question_set(&self) -> QS;
    }
}
