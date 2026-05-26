use use_test::case::TestCase;
use use_test::expectation::TestExpectation;
use use_test::outcome::TestOutcome;

fn main() {
    let case = TestCase::new("empty string trims to empty");
    let expectation = TestExpectation::new("", "".trim());
    let outcome = TestOutcome::Passed;

    assert_eq!(case.name(), "empty string trims to empty");
    assert_eq!(expectation.expected(), expectation.actual());
    assert!(outcome.is_success());
}
