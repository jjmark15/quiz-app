use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct QuestionSet {
    name: String,
    id: u64,
}

impl QuestionSet {
    pub(crate) fn new(name: String, id: u64) -> QuestionSet {
        QuestionSet { name, id }
    }
}
