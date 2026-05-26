# use-test-fixture

Fixture metadata and scope vocabulary for `RustUse` testing primitives.

## Experimental

`use-test-fixture` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_fixture::{TestFixture, TestFixtureScope};

let fixture = TestFixture::new("temp-dir", TestFixtureScope::Case);

assert_eq!(fixture.name(), "temp-dir");
assert_eq!(fixture.scope(), TestFixtureScope::Case);
```

## Scope

- Fixture names.
- Case, suite, module, and session scopes.

## Non-goals

- Setup execution, teardown execution, async behavior, resource allocation, or dependency injection.

## License

Licensed under either Apache-2.0 or MIT.
