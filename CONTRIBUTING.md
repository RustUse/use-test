# Contributing

Thanks for helping improve `use-test`.

This workspace keeps testing vocabulary small, dependency-light, and primitive-focused. Contributions should preserve the boundary that `use-test` is not a test runner, framework, mocking system, snapshot storage engine, benchmark tool, or CI integration.

Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Add focused tests for any new helper behavior and keep public API additions documented in the relevant crate README.
