# use-test-report

Lightweight report summary primitives for `RustUse` testing vocabulary.

## Experimental

`use-test-report` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_report::TestReport;

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
```

## Scope

- Suite-level summary counts.
- Success-rate and aggregate success helpers.

## Non-goals

- Report rendering, JUnit XML, coverage reports, CI publishing, or persistent storage.

## License

Licensed under either Apache-2.0 or MIT.
