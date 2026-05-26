# use-test-case

Named test case metadata for `RustUse` testing vocabulary.

## Experimental

`use-test-case` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_case::{TestCase, TestCaseData};

let case = TestCase::with_description("trim", "trims surrounding whitespace");
let data = TestCaseData::new("trim spaces", " value ", "value");

assert_eq!(case.name(), "trim");
assert_eq!(case.description(), Some("trims surrounding whitespace"));
assert_eq!(data.input(), &" value ");
```

## Scope

- Named case metadata.
- Optional descriptions.
- Generic input/expected case data.

## Non-goals

- Test discovery, execution, parameter expansion, or fixture setup.

## License

Licensed under either Apache-2.0 or MIT.
