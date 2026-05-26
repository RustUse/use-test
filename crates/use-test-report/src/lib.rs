#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Lightweight test report summary counts.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct TestReport {
    pub suite_name: Option<String>,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub errored: usize,
}

impl TestReport {
    pub const fn new() -> Self {
        Self {
            suite_name: None,
            total: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            errored: 0,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn success_rate(&self) -> Option<f64> {
        if self.total == 0 {
            None
        } else {
            Some(self.passed as f64 / self.total as f64)
        }
    }

    pub const fn is_success(&self) -> bool {
        self.failed == 0 && self.errored == 0
    }
}

#[cfg(test)]
mod tests {
    use super::TestReport;

    #[test]
    fn new_report_is_empty() {
        let report = TestReport::new();

        assert_eq!(report.suite_name, None);
        assert_eq!(report.total, 0);
        assert_eq!(report.success_rate(), None);
        assert!(report.is_success());
        assert_eq!(TestReport::default(), report);
    }

    #[test]
    fn computes_success_rate() {
        let report = TestReport {
            suite_name: Some("core".to_string()),
            total: 4,
            passed: 3,
            failed: 1,
            skipped: 0,
            errored: 0,
        };

        assert_eq!(report.success_rate(), Some(0.75));
        assert!(!report.is_success());
    }

    #[test]
    fn errored_report_is_not_successful() {
        let report = TestReport {
            suite_name: None,
            total: 1,
            passed: 0,
            failed: 0,
            skipped: 0,
            errored: 1,
        };

        assert!(!report.is_success());
    }
}
