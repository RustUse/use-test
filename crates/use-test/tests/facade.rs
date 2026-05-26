use use_test::{assertion, case, expectation, fixture, outcome, report, snapshot, status, suite};

#[test]
fn facade_exposes_all_child_modules() {
    let assertion = assertion::TestAssertion::with_label(
        assertion::TestAssertionKind::Equal,
        "expected value matches",
    );
    let expectation = expectation::TestExpectation::new(1, 1);
    let case = case::TestCase::new("integer equality");
    let data = case::TestCaseData::new("trim", " value ", "value");
    let suite = suite::TestSuite::with_case_count("core", 1);
    let outcome = outcome::TestOutcome::Passed;
    let status = status::TestStatus::Finished;
    let fixture = fixture::TestFixture::new("temp-dir", fixture::TestFixtureScope::Case);
    let snapshot_id = snapshot::TestSnapshotId::new("trim-empty-v1");
    let summary = report::TestReport {
        suite_name: Some("core".to_string()),
        total: 1,
        passed: 1,
        failed: 0,
        skipped: 0,
        errored: 0,
    };

    assert_eq!(assertion.label(), Some("expected value matches"));
    assert_eq!(expectation.expected(), expectation.actual());
    assert_eq!(case.name(), "integer equality");
    assert_eq!(data.expected(), &"value");
    assert_eq!(suite.case_count(), 1);
    assert!(outcome.is_success());
    assert_eq!(status, status::TestStatus::Finished);
    assert_eq!(fixture.scope(), fixture::TestFixtureScope::Case);
    assert_eq!(snapshot_id.as_str(), "trim-empty-v1");
    assert_eq!(summary.success_rate(), Some(1.0));
}
