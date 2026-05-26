# RustUse/use-test Crate Template

Use this checklist when adding a focused crate to `use-test`.

## Target Layout

```text
crates/use-test-example/
  Cargo.toml
  README.md
  src/
    lib.rs
```

## Cargo.toml Pattern

```toml
[package]
name = "use-test-example"
description = "Primitive example metadata for RustUse testing vocabulary."
publish = true
authors.workspace = true
version.workspace = true
edition.workspace = true
homepage.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
readme = "README.md"
documentation = "https://docs.rs/use-test-example"
keywords = ["test", "metadata", "rustuse"]
categories = ["development-tools", "data-structures"]

[lints]
workspace = true
```

## src/lib.rs Pattern

```rust
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExampleMetadata {
    pub name: String,
}
```

Keep focused crates dependency-free unless a later release has a clear interoperability reason to add optional feature-gated support.
