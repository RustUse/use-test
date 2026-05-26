# Releasing

This repository follows the standard RustUse first-publish flow for a small facade workspace.

## Current release state

`use-test` publishes focused child crates first, then publishes the `use-test` facade after crates.io index propagation.

## First publish order

1. `use-test-assertion`
2. `use-test-expectation`
3. `use-test-case`
4. `use-test-suite`
5. `use-test-outcome`
6. `use-test-status`
7. `use-test-fixture`
8. `use-test-snapshot`
9. `use-test-report`
10. `use-test`

## Local readiness

Run the validation path from the repository root before publishing:

```sh
make release-readiness
```

If `make` is unavailable, run the equivalent Cargo commands listed in `README.md` and dry-run each focused crate before the facade.
