# use-test-suite

Grouped test suite metadata for `RustUse` testing vocabulary.

## Experimental

`use-test-suite` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_suite::TestSuite;

let suite = TestSuite::with_description("strings", "string helper cases");

assert_eq!(suite.name(), "strings");
assert_eq!(suite.description(), Some("string helper cases"));
assert_eq!(suite.case_count(), 0);
```

## Scope

- Suite names.
- Optional descriptions.
- Lightweight case counts.

## Non-goals

- Test grouping execution, ordering, scheduling, or discovery.

## License

Licensed under either Apache-2.0 or MIT.
