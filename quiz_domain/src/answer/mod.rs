use std::fmt::Debug;

pub use basic_answer::*;
#[cfg(test)]
pub use tests::*;

mod basic_answer;

pub trait Answer: Eq + PartialEq + Debug + Clone + Default {
    fn satisfied_by(&self, answer: &Self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};

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
}
