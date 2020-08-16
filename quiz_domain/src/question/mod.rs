use std::fmt::Debug;

pub use simple_question::*;

use crate::id::ModelID;
use crate::Answer;

mod simple_question;

pub trait Question: Debug + Clone {
    type QuestionID: ModelID;
    type QuestionSetID: ModelID;
    type QuestionAnswer: Answer;

    fn id(&self) -> &Self::QuestionID;

    fn question_set_id(&self) -> &Self::QuestionSetID;

    fn query_message(&self) -> &String;

    fn answer(&self) -> &Self::QuestionAnswer;

    fn answered_by(&self, answer: &Self::QuestionAnswer) -> bool;
}
