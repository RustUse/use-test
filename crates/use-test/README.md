# use-test

Primitive testing vocabulary facade for `RustUse`.

## Experimental

`use-test` is experimental while it remains below `0.3.0`.

## Example

```rust
use use_test::case::TestCase;
use use_test::expectation::TestExpectation;
use use_test::outcome::TestOutcome;

let case = TestCase::new("empty string trims to empty");
let expectation = TestExpectation::new("", "".trim());
let outcome = TestOutcome::Passed;

assert_eq!(case.name(), "empty string trims to empty");
assert_eq!(expectation.expected(), expectation.actual());
assert!(outcome.is_success());
```

## Scope

- Module-style aliases for every focused `use-test-*` child crate.
- Primitive metadata for assertions, expectations, cases, suites, outcomes, statuses, fixtures, snapshots, and reports.

## Non-goals

- Running tests, assertion macros, mocking, snapshot storage, benchmark execution, or CI/reporting integrations.

## License

Licensed under either Apache-2.0 or MIT.
