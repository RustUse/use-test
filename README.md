# RustUse/use-test

`use-test` is the RustUse facade/set for primitive testing vocabulary. RustUse is a collection of composable, dependency-light Rust utility crates, and this workspace provides small metadata types for assertions, expectations, test cases, suites, outcomes, statuses, fixtures, snapshots, and lightweight reports.

This repository is experimental while it is below `0.3.0`. APIs are intentionally small and may change as the RustUse testing vocabulary settles.

## Purpose

`use-test` is a primitive vocabulary layer for projects that need to describe testing concepts without adopting a runner, framework, mocking system, snapshot engine, or CI integration.

## What It Is

- Primitive Rust testing metadata types.
- Focused child crates with small public APIs.
- A facade crate that exposes child crates through module-style aliases.
- A shared vocabulary for assertions, expectations, cases, suites, outcomes, statuses, fixtures, snapshots, and reports.

## What It Is Not

- Not a test runner.
- Not a test framework.
- Not a replacement for Rust's built-in test system.
- Not a replacement for crates like `proptest`, `rstest`, `insta`, or `criterion`.
- Not a mocking, fake, stub, or spy framework.
- Not a snapshot file storage engine.
- Not a benchmark framework.
- Not a CI system or reporting integration.

## Child Crate Naming

All child crates in this workspace use the `use-test-*` naming pattern. This keeps testing-specific primitives from colliding with broader RustUse vocabulary crates while making their relationship to the `use-test` facade explicit.

## Crates

| Crate                  | Description                                         |
| ---------------------- | --------------------------------------------------- |
| `use-test`             | Facade over the focused test primitive crates.      |
| `use-test-assertion`   | Assertion kind and assertion metadata primitives.   |
| `use-test-expectation` | Expected-vs-actual value primitives.                |
| `use-test-case`        | Named test case and case data metadata.             |
| `use-test-suite`       | Grouped test suite metadata.                        |
| `use-test-outcome`     | Stable pass/fail/skip/error outcome vocabulary.     |
| `use-test-status`      | Lifecycle status vocabulary separate from outcomes. |
| `use-test-fixture`     | Fixture metadata and fixture scope vocabulary.      |
| `use-test-snapshot`    | Snapshot identity, version, and status primitives.  |
| `use-test-report`      | Lightweight test report summary primitives.         |

## Installation

```toml
[dependencies]
use-test = "0.1.0"
```

Focused crates can also be used directly:

```toml
[dependencies]
use-test-case = "0.1.0"
use-test-outcome = "0.1.0"
```

## Basic Usage

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

- Assertion, expectation, test case, and test suite metadata.
- Outcome and lifecycle status vocabulary.
- Fixture scope metadata.
- Snapshot identity/version/status labels without file I/O.
- Lightweight summary counts and success-rate helpers.

## Non-goals

- Running tests, scheduling tests, discovering tests, or integrating with test harnesses.
- Assertion macros, procedural macros, property testing, fuzzing, or benchmark execution.
- Mocking, spying, fake services, dependency injection, or fixture setup/teardown execution.
- Snapshot file storage, golden-file updates, or snapshot review commands.
- External report formats such as JUnit XML in v0.1.0.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

For release-readiness checks, also run the repository tasks in the `Makefile`.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
