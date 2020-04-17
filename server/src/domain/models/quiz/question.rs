use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait ModelIDInterface<'a>:
    Eq + PartialEq + Deserialize<'a> + Serialize + Clone + Debug
{
    fn value(&self) -> String;
}

pub trait QuestionSetInterface<'a, ID: ModelIDInterface<'a>>:
    Debug + Deserialize<'a> + Serialize + Clone
{
    fn id(&self) -> &ID;

    fn name(&self) -> &String;

    fn with_id(name: String, id: ID) -> Self;

    fn with_name(name: String) -> Self;
}
