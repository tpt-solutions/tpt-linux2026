# Contributing to tpt-linux2026

Thanks for your interest! This document explains how to get a change merged.

## Getting started

```sh
git clone https://github.com/tpt-solutions/tpt-linux2026
cd tpt-linux2026
cargo build --workspace
cargo test  --workspace
```

## Workflow

1. File or claim an issue / a `todo.md` checklist item.
2. Create a feature branch off `main`.
3. Make your change. Keep `unsafe` confined to `ffi.rs`/`bindings.rs`.
4. Ensure `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`,
   and `cargo deny check` all pass.
5. Open a PR. CI must be green before merge (branch protection on `main`).

## Engineering standards

- **Edition 2024**, MSRV `1.85.0` (see `rust-toolchain.toml`).
- **No `unsafe` in public APIs** except where explicitly documented.
- **`no_std` where possible** for kernel-adjacent crates.
- **Tests required** for every public function with non-trivial logic.
- **Licensing:** all crates are `MIT OR Apache-2.0`, except
  `tpt-l-kprobe-harness` which adds `GPL-2.0`.

## Commit messages

Use imperative, concise subjects: `feat(sat-solver): add VSIDS heuristic`.

## Code of conduct

Be respectful. Harassment of any kind is not tolerated.
