pub use basic_answer::*;
#[cfg(test)]
pub use tests::*;

mod basic_answer;

pub trait Answer {
    fn satisfied_by(&self, answer: &Self) -> bool;
}

#[cfg(test)]
mod tests {
    use mockall::{mock, predicate::*};

    use super::*;

    mock! {
        pub Answer {}

        trait Answer {
            fn satisfied_by(&self, answer: &MockAnswer) -> bool;
        }
    }
}
