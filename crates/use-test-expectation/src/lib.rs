#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A simple expected-vs-actual pair.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TestExpectation<T> {
    pub expected: T,
    pub actual: T,
}

impl<T> TestExpectation<T> {
    pub const fn new(expected: T, actual: T) -> Self {
        Self { expected, actual }
    }

    pub const fn expected(&self) -> &T {
        &self.expected
    }

    pub const fn actual(&self) -> &T {
        &self.actual
    }

    pub fn into_parts(self) -> (T, T) {
        (self.expected, self.actual)
    }
}

#[cfg(test)]
mod tests {
    use super::TestExpectation;

    #[test]
    fn stores_expected_and_actual_values() {
        let expectation = TestExpectation::new("expected".to_string(), "actual".to_string());

        assert_eq!(expectation.expected(), "expected");
        assert_eq!(expectation.actual(), "actual");
    }

    #[test]
    fn splits_into_parts() {
        let expectation = TestExpectation::new(1, 2);

        assert_eq!(expectation.into_parts(), (1, 2));
    }
}
