# tpt-l-ebpf-common

Shared eBPF utilities for the `tpt-linux2026` workspace.

- `program_name!` and default sizing constants in [`macros`].
- Safe [Aya](https://aya-rs.dev) loader helpers in [`loader`] (Linux only).

This crate is `publish = false` and is consumed only by the eBPF crates in the
workspace.

## License

MIT OR Apache-2.0.
