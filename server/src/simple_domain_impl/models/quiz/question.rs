use serde::{Deserialize, Serialize};
use warp::reply::Response;

use crate::domain::models::quiz::question::{ModelIDInterface, QuestionSetInterface};

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct ModelIDImpl {
    id: u64,
}

impl ModelIDInterface<'_> for ModelIDImpl {
    fn value(&self) -> String {
        format!("{}", self.id)
    }
}

impl ModelIDImpl {
    fn new(value: u64) -> Self {
        ModelIDImpl { id: value }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuestionSetImpl {
    #[serde(flatten)]
    id: ModelIDImpl,
    name: String,
}

impl QuestionSetInterface<'_, ModelIDImpl> for QuestionSetImpl {
    fn id(&self) -> &ModelIDImpl {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn with_id(name: String, id: ModelIDImpl) -> Self {
        QuestionSetImpl { name, id }
    }

    fn with_name(name: String) -> Self {
        QuestionSetImpl {
            name,
            id: ModelIDImpl::new(0),
        }
    }
}

impl warp::Reply for QuestionSetImpl {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
