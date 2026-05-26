#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Test lifecycle status vocabulary distinct from outcome vocabulary.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TestStatus {
    #[default]
    Pending,
    Running,
    Finished,
    Skipped,
    Ignored,
    Disabled,
}

#[cfg(test)]
mod tests {
    use super::TestStatus;

    #[test]
    fn pending_is_default_status() {
        assert_eq!(TestStatus::default(), TestStatus::Pending);
    }

    #[test]
    fn status_is_distinct_from_outcome() {
        assert_eq!(TestStatus::Running, TestStatus::Running);
        assert_ne!(TestStatus::Running, TestStatus::Finished);
    }
}
