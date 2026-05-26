# use-test-assertion

Assertion metadata primitives for `RustUse` testing vocabulary.

## Experimental

`use-test-assertion` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_assertion::{TestAssertion, TestAssertionKind};

let assertion = TestAssertion::with_label(TestAssertionKind::Equal, "value matches");

assert_eq!(assertion.kind(), TestAssertionKind::Equal);
assert_eq!(assertion.label(), Some("value matches"));
```

## Scope

- Assertion kind labels.
- Optional assertion labels for metadata and reporting.

## Non-goals

- Assertion macros, test execution, diffing, or panic behavior.

## License

Licensed under either Apache-2.0 or MIT.
