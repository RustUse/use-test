# use-test-status

Lifecycle status vocabulary for `RustUse` testing primitives.

## Experimental

`use-test-status` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_status::TestStatus;

let status = TestStatus::default();

assert_eq!(status, TestStatus::Pending);
```

## Scope

- Pending, running, finished, skipped, ignored, and disabled lifecycle labels.

## Non-goals

- Outcome pass/fail semantics or runner state machines.

## License

Licensed under either Apache-2.0 or MIT.