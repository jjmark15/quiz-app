use mockall::mock;

use quiz_domain::{ModelID, QuestionSetInterface, QuizServiceInterface};

mock! {
    pub QuizService<MID, QS>
     where
        MID: 'static +  ModelID,
        QS: 'static +  QuestionSetInterface<ID=MID>,
     {}

    trait QuizServiceInterface {
        type QuestionSetID = MID;
        type QuestionSet = QS;

        fn get_example_question_set() -> QS;
    }
}
