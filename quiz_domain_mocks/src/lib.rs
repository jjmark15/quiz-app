use std::fmt::Debug;

use mockall::mock;

use quiz_domain::{Answer, ModelID, QuestionSetInterface, QuizServiceInterface, Uuid};

mock! {
    pub ModelID {}

    trait ModelID {
        fn value(&self) -> Uuid;

        fn new(value: Uuid) -> Self;

        fn random() -> Self;
    }

    trait Clone {
        fn clone(&self) -> MockModelID;
    }

    trait PartialEq {
        fn eq(&self, other: &MockModelID) -> bool;
    }

    trait Eq {}

    trait Debug {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result;
    }
}

mock! {
    pub QuestionSet {}

    trait QuestionSetInterface {
        type ID = MockModelID;

        fn id(&self) -> MockModelID;

        fn name(&self) -> &String;

        fn with_id(id: MockModelID, name: String) -> Self;

        fn new(name: String) -> Self {
            Self::with_id(MockModelID::random(), name)
        }
    }

    trait Clone {
        fn clone(&self) -> MockQuestionSet;
    }

    trait PartialEq {
        fn eq(&self, other: &MockQuestionSet) -> bool;
    }

    trait Eq {}

    trait Debug {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result;
    }
}

mock! {
    pub QuizService {}

    trait QuizServiceInterface {
        type QuestionSetID = MockModelID;
        type QuestionSet = MockQuestionSet;

        fn get_example_question_set() -> MockQuestionSet;
    }
}

mock! {
    pub Answer {}

    trait Answer {
        fn satisfied_by(&self, answer: &MockAnswer) -> bool;
    }

    trait Clone {
        fn clone(&self) -> MockAnswer;
    }

    trait PartialEq {
        fn eq(&self, other: &MockAnswer) -> bool;
    }

    trait Eq {}

    trait Debug {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result;
    }
}
