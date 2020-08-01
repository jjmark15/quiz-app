use crate::entities::Answer;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct BasicAnswer {
    value: String,
}

impl Answer for BasicAnswer {
    fn satisfied_by(&self, answer: &Self) -> bool {
        self.eq(answer)
    }
}

impl BasicAnswer {
    pub fn new(value: String) -> Self {
        BasicAnswer { value }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::entities::Answer;

    use super::*;

    fn basic_answer(value: String) -> BasicAnswer {
        BasicAnswer::new(value)
    }

    #[test]
    fn is_satisfied_by_exactly_equal_answer() {
        let answer = basic_answer("answer".to_string());
        asserting("is satisfied by exactly equal answer")
            .that(&answer.satisfied_by(&answer.clone()))
            .is_true()
    }

    #[test]
    fn is_not_satisfied_by_different_answer() {
        let answer = basic_answer("answer".to_string());
        asserting("is not satisfied by different answer")
            .that(&answer.satisfied_by(&basic_answer("other answer".to_string())))
            .is_false()
    }

    #[test]
    fn is_not_satisfied_by_incorrect_case() {
        let answer = basic_answer("answer".to_string());
        asserting("is not satisfied by incorrect case")
            .that(&answer.satisfied_by(&basic_answer("Answer".to_string())))
            .is_false()
    }

    #[test]
    fn is_not_satisfied_by_extra_whitespace() {
        let answer = basic_answer("answer".to_string());
        asserting("is not satisfied by extra whitespace")
            .that(&answer.satisfied_by(&basic_answer("answer ".to_string())))
            .is_false()
    }
}
