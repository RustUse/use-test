#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Named test case metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TestCase {
    pub name: String,
    pub description: Option<String>,
}

impl TestCase {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }

    pub fn with_description(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Generic test case data without execution behavior.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TestCaseData<I, O> {
    pub name: String,
    pub input: I,
    pub expected: O,
}

impl<I, O> TestCaseData<I, O> {
    pub fn new(name: impl Into<String>, input: I, expected: O) -> Self {
        Self {
            name: name.into(),
            input,
            expected,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn input(&self) -> &I {
        &self.input
    }

    pub const fn expected(&self) -> &O {
        &self.expected
    }

    pub fn into_parts(self) -> (String, I, O) {
        (self.name, self.input, self.expected)
    }
}

#[cfg(test)]
mod tests {
    use super::{TestCase, TestCaseData};

    #[test]
    fn creates_named_case() {
        let case = TestCase::new("basic");

        assert_eq!(case.name(), "basic");
        assert_eq!(case.description(), None);
    }

    #[test]
    fn creates_described_case() {
        let case = TestCase::with_description("trim", "removes whitespace");

        assert_eq!(case.name(), "trim");
        assert_eq!(case.description(), Some("removes whitespace"));
    }

    #[test]
    fn stores_case_data() {
        let data = TestCaseData::new("length", "abc", 3);

        assert_eq!(data.name(), "length");
        assert_eq!(data.input(), &"abc");
        assert_eq!(data.expected(), &3);
        assert_eq!(data.into_parts(), ("length".to_string(), "abc", 3));
    }
}
