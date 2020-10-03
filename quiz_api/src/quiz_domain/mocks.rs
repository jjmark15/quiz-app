use mockall::mock;

use quiz_domain::{ExampleQuizObjectsService, ModelId, QuestionSet, Uuid};

mock! {
    pub ExampleQuizObjectsService<MID, QS>
    where
        MID: 'static +  ModelId + Default,
        QS: 'static +  QuestionSet<QuestionSetId=MID>,
    {}

    trait ExampleQuizObjectsService {
        type QuestionSetID = MID;
        type QuestionSetType = QS;

        fn get_example_question_set(&self) -> QS;
    }
}

mock! {
    pub QuestionSet<QSID: 'static +  ModelId> {}
    
    trait QuestionSet {
        type QuestionSetId = QSID;
    
        fn id(&self) -> QSID;
    
        fn name(&self) -> &String;
    
        fn with_id(id: QSID, name: String) -> Self;
    
        fn new(name: String) -> Self;
    }
}

mock! {
    pub ModelId {}
    
    pub trait ModelId {
        fn value(&self) -> Uuid;
    
        fn new(value: Uuid) -> Self;
    
        fn random() -> Self;
    }
}
