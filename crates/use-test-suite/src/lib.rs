#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Grouped test case metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TestSuite {
    pub name: String,
    pub description: Option<String>,
    pub case_count: usize,
}

impl TestSuite {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            case_count: 0,
        }
    }

    pub fn with_case_count(name: impl Into<String>, case_count: usize) -> Self {
        Self {
            name: name.into(),
            description: None,
            case_count,
        }
    }

    pub fn with_description(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            case_count: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub const fn case_count(&self) -> usize {
        self.case_count
    }
}

#[cfg(test)]
mod tests {
    use super::TestSuite;

    #[test]
    fn creates_named_suite() {
        let suite = TestSuite::new("core");

        assert_eq!(suite.name(), "core");
        assert_eq!(suite.description(), None);
        assert_eq!(suite.case_count(), 0);
    }

    #[test]
    fn creates_suite_with_case_count() {
        let suite = TestSuite::with_case_count("core", 3);

        assert_eq!(suite.name(), "core");
        assert_eq!(suite.case_count(), 3);
    }

    #[test]
    fn creates_described_suite() {
        let suite = TestSuite::with_description("core", "core cases");

        assert_eq!(suite.description(), Some("core cases"));
    }
}
