# use-test-snapshot

Snapshot identity and status primitives for `RustUse` testing vocabulary.

## Experimental

`use-test-snapshot` is experimental while `use-test` remains below `0.3.0`.

## Example

```rust
use use_test_snapshot::{TestSnapshotId, TestSnapshotStatus};

let snapshot = TestSnapshotId::new("homepage-empty-state-v1");

assert_eq!(snapshot.as_str(), "homepage-empty-state-v1");
assert_eq!(TestSnapshotStatus::default(), TestSnapshotStatus::Missing);
```

## Scope

- Snapshot identifiers, names, and versions.
- Missing, created, matched, changed, accepted, and rejected status labels.

## Non-goals

- File I/O, golden-file writing, update commands, review workflows, or storage policy.

## License

Licensed under either Apache-2.0 or MIT.
