#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Stable vocabulary for completed test outcomes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TestOutcome {
    Passed,
    Failed,
    Skipped,
    Errored,
    ExpectedFailure,
    UnexpectedPass,
}

impl TestOutcome {
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Passed | Self::ExpectedFailure)
    }

    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Failed | Self::Errored | Self::UnexpectedPass)
    }

    pub const fn is_terminal(self) -> bool {
        match self {
            Self::Passed
            | Self::Failed
            | Self::Skipped
            | Self::Errored
            | Self::ExpectedFailure
            | Self::UnexpectedPass => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TestOutcome;

    #[test]
    fn classifies_success_outcomes() {
        assert!(TestOutcome::Passed.is_success());
        assert!(TestOutcome::ExpectedFailure.is_success());
        assert!(!TestOutcome::Skipped.is_success());
    }

    #[test]
    fn classifies_failure_outcomes() {
        assert!(TestOutcome::Failed.is_failure());
        assert!(TestOutcome::Errored.is_failure());
        assert!(TestOutcome::UnexpectedPass.is_failure());
        assert!(!TestOutcome::ExpectedFailure.is_failure());
    }

    #[test]
    fn all_outcomes_are_terminal() {
        let outcomes = [
            TestOutcome::Passed,
            TestOutcome::Failed,
            TestOutcome::Skipped,
            TestOutcome::Errored,
            TestOutcome::ExpectedFailure,
            TestOutcome::UnexpectedPass,
        ];

        assert!(outcomes.into_iter().all(TestOutcome::is_terminal));
    }
}
