#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Primitive assertion categories.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TestAssertionKind {
    Equal,
    NotEqual,
    Contains,
    DoesNotContain,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    ApproxEqual,
    IsTrue,
    IsFalse,
    IsSome,
    IsNone,
    IsOk,
    IsErr,
}

/// Assertion metadata without assertion execution.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TestAssertion {
    pub kind: TestAssertionKind,
    pub label: Option<String>,
}

impl TestAssertion {
    pub const fn new(kind: TestAssertionKind) -> Self {
        Self { kind, label: None }
    }

    pub fn with_label(kind: TestAssertionKind, label: impl Into<String>) -> Self {
        Self {
            kind,
            label: Some(label.into()),
        }
    }

    pub const fn kind(&self) -> TestAssertionKind {
        self.kind
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::{TestAssertion, TestAssertionKind};

    #[test]
    fn creates_unlabeled_assertion() {
        let assertion = TestAssertion::new(TestAssertionKind::IsOk);

        assert_eq!(assertion.kind(), TestAssertionKind::IsOk);
        assert_eq!(assertion.label(), None);
    }

    #[test]
    fn creates_labeled_assertion() {
        let assertion = TestAssertion::with_label(TestAssertionKind::Contains, "has item");

        assert_eq!(assertion.kind(), TestAssertionKind::Contains);
        assert_eq!(assertion.label(), Some("has item"));
    }
}
