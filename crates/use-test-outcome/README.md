# use-test-outcome

Stable test outcome vocabulary for `RustUse` testing primitives.

## Experimental

`use-test-outcome` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_outcome::TestOutcome;

assert!(TestOutcome::Passed.is_success());
assert!(TestOutcome::Failed.is_failure());
assert!(TestOutcome::Skipped.is_terminal());
```

## Scope

- Passed, failed, skipped, errored, expected-failure, and unexpected-pass vocabulary.
- Small helper predicates over outcome semantics.

## Non-goals

- Lifecycle status, runner state, retry behavior, or reporting formats.

## License

Licensed under either Apache-2.0 or MIT.
