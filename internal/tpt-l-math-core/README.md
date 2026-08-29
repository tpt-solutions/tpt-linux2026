# tpt-l-math-core

Shared, `no_std`-compatible mathematics used across the `tpt-linux2026`
workspace:

- **`sat`** — low-level SAT primitives (variables, literals, clauses, CNF) used
  by `tpt-l-sat-solver`.
- **`color`** — Rec. 2100 transfer functions (PQ / HLG) and color-space helpers
  used by `tpt-l-wayland-hdr-vrr-kit`.

This crate has no dependencies and does not require the standard library.

## License

MIT OR Apache-2.0.
