# use-test-expectation

Expected-vs-actual primitives for `RustUse` testing vocabulary.

## Experimental

`use-test-expectation` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_expectation::TestExpectation;

let expectation = TestExpectation::new(4, 2 + 2);

assert_eq!(expectation.expected(), &4);
assert_eq!(expectation.actual(), &4);
```

## Scope

- Generic expected and actual value pairing.
- Accessors and ownership-preserving decomposition.

## Non-goals

- Diffing, formatting, assertion execution, or comparison policy.

## License

Licensed under either Apache-2.0 or MIT.
